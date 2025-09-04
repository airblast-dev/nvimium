use std::mem::MaybeUninit;

#[cold]
fn cold() {}

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
        mod builder {
          use crate::macros::{ utils::{count_tts}, constified::{strings_len_max, count_unique_chars}, hash_face };

          #[allow(unused_labels, unreachable_code)]
          pub(super) const FIELDS: [&'static str; count_tts!($($field_name),*)] = [$(
              '__nvimium_internal_label: {
                  $( break '__nvimium_internal_label $renamed; )?
                  ::std::stringify!($field_name)
              }
          ),*];

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

macro_rules! gen_field {
    (
        #[nv_enum] $field_name:ident: $field_ty:ty
    ) => {
        $crate::nvim_types::ThinString<'static>
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

macro_rules! gen_funcs {
    (@IDX=$idx:expr;) => {};
    (
        @IDX=$idx:expr;
        #[nv_enum] $field_name:ident: $field_ty:ty,
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

masked_builder! {
    struct A {
        a: u8 = "hello",
        b: u8,
    }
}
