use std::mem::MaybeUninit;

#[cold]
fn cold() {}

/// just an internal function to avoid logic inside a macro
pub(crate) unsafe fn assign_field<T>(
    idx: usize,
    cur_mask: &mut u64,
    mask_offsets: &'static [u64],
    field: &mut MaybeUninit<T>,
    value: T,
) {
    let mask: u64 = 1 << mask_offsets[idx];
    if *cur_mask & mask == mask {
        cold();
        unsafe { field.assume_init_drop() };
    } else {
        *cur_mask |= mask;
    }

    field.write(value);
}

/// Create a struct with masked fields
///
/// # The mask
///
/// A mask field with a value of [`u64`] is prepended to the provided fields. For each field a bit
/// is set once a value is assigned.
///
/// # Assigning values/setters
///
/// Since there is no way to specify that assigning a value to a field should set a specific bit in
/// the structs mask we are required to use setters instead. These setters handle the mask and drop logic of
/// the arguments so from a user perspective its just a function with an argument.
///
/// # Builder module
///
/// When a struct with masked fields is created a builder module is defined.
/// The builder module provides various const values to use when manually implementing a setter
/// function.
macro_rules! masked_builder {
    ($(#[$struct_attr:meta])* $struct_vis:vis struct $struct_name:ident {

    }) => {
        $( #[$struct_attr] )*
        $struct_vis struct $struct_name {
            mask: u64,
        }
    };
    ($(#[$struct_attr:meta])* $struct_vis:vis struct $struct_name:ident $(<$lf:lifetime>)? {
        $(
            $(#[builder($( $builder_attr:tt )+)])?
            //$(#[$($extra_tt:meta)+])*
            $field_name:ident: $field_ty:ty $(= $renamed:literal)?,
        )+
    }) => {
        /// The builder module for a struct with masked fields.
        mod builder {
          use crate::macros::{ utils::{count_tts}, constified::{strings_len_max, count_unique_chars}, hash_face };

          /// All of the fields of the struct stringified.
          #[allow(unused_labels, unreachable_code)]
          pub(super) const FIELDS: [&'static str; count_tts!($($field_name),*)] = [$(
              '__nvimium_internal_label: {
                  $( break '__nvimium_internal_label $renamed; )?
                  ::std::stringify!($field_name)
              }
          ),*];

          /// Contains the bit indexes to set when assigning a value to the structs fields.
          ///
          /// The values are ordered in the same way the structs fields are declared.
          /// These values should be used directly and instead should be used by performing shifts
          /// on 1 such as `1 << MASK_OFFSETS[2]`.
          pub(super) const MASK_OFFSETS: [u64; const {FIELDS.len()}] = hash_face::fields_to_bit_shifts::<
              { FIELDS.len() }, { count_unique_chars(&FIELDS) }, { strings_len_max(&FIELDS) }
            >(&FIELDS);

        }

        $(#[$struct_attr])*
        $struct_vis struct $struct_name $(<$lf>)? {
            mask: u64,
            $(
                //$(#[$($extra_tt)+])*
                $field_name: ::std::mem::MaybeUninit<$crate::macros::masked_builder::gen_field!{ $( #[$( $builder_attr )+] )? $field_name: $field_ty }>,
            )*
        }

        impl $(<$lf>)? $struct_name $(<$lf>)? {
            $crate::macros::masked_builder::gen_funcs!(@IDX=0; $($( #[$( $builder_attr )+] )? $field_name: $field_ty,)*);
        }
    };
}
pub(crate) use masked_builder;

/// Decides which type to actually store whilst acknowledging the builder attribute on the field.
macro_rules! gen_field {
    (
        #[nv_str_enum] $field_name:ident: $field_ty:ty
    ) => {
        $crate::nvim_types::ThinString<'static>
    };
    (
        #[nv_obj_ref_enum] $field_name:ident: $field_ty:ty
    ) => {
        $crate::nvim_types::object::ObjectRef<'static>
    };
    (
        #[skip] $field_name:ident: $field_ty:ty
    ) => {
        $field_ty
    };
    (
        #[into] $field_name:ident: $field_ty:ty
    ) => {
        $field_ty
    };
    (
        $(#[nv_str])? $field_name:ident: $field_ty:ty
    ) => {
        $field_ty
    };
}
pub(crate) use gen_field;

/// Generate the functions for a struct with masked fields
///
/// Accepts the following syntax of `@IDX=#1; #[#2] field_name field_ident...`
/// #1 = The index to use when looking up the Nth bit to set when assigning this field
/// #2 = One of the following attributes [nv_str_enum, nv_str, skip, into]
///
/// # nv_str_enum
///
/// The function is modified to accept a neovim string enum where each variant is represented as a
/// string. This is the same as nv_str but instead of using [`AsThinString`] it calls a method of
/// `as_enum_str` where it is required to return a [`ThinString<'static>`].
///
/// # nv_str
///
/// The function is modified to accept a reference to anything that implements
/// [`AsThinString`].
///
/// # skip
///
/// The function is not implemented. Used when there are legacy fields present or we want to
/// introduce specific complex bounds on the function argument.
///
/// # into
///
/// The function is modified to accept an argument of `Into<field_type>`.
///
/// [AsThinString](crate::nvim_types::AsThinString)
/// [ThinString](crate::nvim_types::ThinString)
macro_rules! gen_funcs {
    (@IDX=$idx:expr;) => {};
    (
        @IDX=$idx:expr;
        #[nv_str_enum] $field_name:ident: $field_ty:ty,
        $($next:tt)*
    ) => {
        pub fn $field_name(&mut self, $field_name: $field_ty) -> &mut Self {
            self.$field_name.write($field_name.as_enum_str());
            self.mask |= 1 << builder::MASK_OFFSETS[$idx];
            self
        }
        $crate::macros::masked_builder::gen_funcs!(@IDX=$idx + 1; $($next)*);
    };
    (
        @IDX=$idx:expr;
        #[nv_obj_ref_enum] $field_name:ident: $field_ty:ty,
        $($next:tt)*
    ) => {
        pub fn $field_name(&mut self, $field_name: $field_ty) -> &mut Self {
            self.$field_name.write($field_name.as_obj_ref());
            self.mask |= 1 << builder::MASK_OFFSETS[$idx];
            self
        }
        $crate::macros::masked_builder::gen_funcs!(@IDX=$idx + 1; $($next)*);
    };
    (
        @IDX=$idx:expr;
        #[into] $field_name:ident: $field_ty:ty,
        $($next:tt)*
    ) => {
        pub fn $field_name<T: ::std::convert::Into<$field_ty>>(&mut self, $field_name: T) -> &mut Self {
            unsafe {
                $crate::macros::masked_builder::assign_field(
                    $idx, &mut self.mask, &builder::MASK_OFFSETS, &mut self.$field_name, $field_name.into()
                );
            }
            self
        }
        $crate::macros::masked_builder::gen_funcs!(@IDX=$idx + 1; $($next)*);
    };
    (
        @IDX=$idx:expr;
        #[nv_str] $field_name:ident: $field_ty:ty,
        $($next:tt)*
    ) => {
        pub fn $field_name<T: ?Sized + $crate::nvim_types::AsThinString>(&mut self, $field_name: &'a T) -> &mut Self {
            unsafe {
                $crate::macros::masked_builder::assign_field(
                    $idx, &mut self.mask, &builder::MASK_OFFSETS, &mut self.$field_name, $field_name.as_thinstr()
                );
            }
            self
        }
        $crate::macros::masked_builder::gen_funcs!(@IDX=$idx + 1; $($next)*);
    };
    (
        @IDX=$idx:expr;
        #[skip] $field_name:ident: $field_ty:ty,
        $($next:tt)*
    ) => {
        $crate::macros::masked_builder::gen_funcs!(@IDX=$idx + 1; $($next)*);
    };
    (
        @IDX=$idx:expr;
        $field_name:ident: $field_ty:ty,
        $($next:tt)*
    ) => {
        pub fn $field_name(&mut self, $field_name: $field_ty) -> &mut Self {
            unsafe {
                $crate::macros::masked_builder::assign_field(
                    $idx, &mut self.mask, &builder::MASK_OFFSETS, &mut self.$field_name, $field_name.into()
                );
            }
            self
        }
        $crate::macros::masked_builder::gen_funcs!(@IDX=$idx + 1; $($next)*);
    };
}
pub(crate) use gen_funcs;
