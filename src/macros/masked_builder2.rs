macro_rules! masked_builder2 {
    (
        $struct_vis:vis struct $struct_name:ident $(<$($lf:lifetime),*>)? {
            $(
                $(#[$attributes:meta])*
                $field_vis:vis $field_name:ident: $field_type:ty,
            )+
        }
    ) => {

        $struct_vis struct $struct_name $(<$($lf),*>)? {
            mask: u64,
            $($field_vis $field_name: $field_type),*
        }

        impl $(<$($lf),*>)? $struct_name $(<$($lf),*>)? {
          const __FIELDS_SLICE: &[&'static str] = &crate::macros::masked_builder2::gen_field_names!(
              $($field_name),+
          );
          const FIELD_COUNT: usize = Self::__FIELDS_SLICE.len();
          const FIELDS: [&str; Self::FIELD_COUNT] = unsafe {* ((Self::__FIELDS_SLICE as *const [&'static str]) as *const [&'static str; Self::FIELD_COUNT]) };
          const FIELD_MAX_LEN: usize = crate::macros::hash_face::strings_len_max(&Self::FIELDS);
          const FIELDS_SUM_LEN: usize = crate::macros::hash_face::strings_len_sum(&Self::FIELDS);
          const MASK_OFFSETS: [usize; Self::FIELD_COUNT] =  crate::macros::hash_face::fields_to_bit_shifts::<
              { Self::FIELD_COUNT }, { Self::FIELDS_SUM_LEN }, { Self::FIELD_MAX_LEN }
            >(&Self::FIELDS);
        }
    };
}

/// Macro for generating stringified fields that allows renaming.
///
/// Basically a helper macro for masked builder
macro_rules! gen_field_names {
    (
        $(
            $(#[$($attr:tt)+])*
            $field_name:ident
        ),+
    ) => {
        [
            $( select_field_attr!($(#[$($attr)+])* $field_name) ),+
        ]
    };
}

pub(crate) use gen_field_names;

/// Checks if the field needs to be renamed before being hashed for the bit index of the field
///
/// uses a pushdown accum and a macro branch to determine if a alternative name was specified.
/// In neovim this is required in a few places where field names and Rust keywords clash.
macro_rules! select_field_attr {
    (@ACC [$($acc:tt)*] #[builder_field(rename = $rename:literal)] $($tt:tt)*) => {
        select_field_attr!(@ACC [$rename] $($tt)* )
    };
    (@ACC [$($acc:tt)*] #[$other_attr:meta] $($tt:tt)*) => {
        select_field_attr!(@ACC [$($acc)*] $($tt)* )
    };
    (@ACC [$($acc:tt)+] $field_name:ident) => {{
        $($acc)+
    }};
    (@ACC [] $field_name:ident) => {
        ::core::stringify!($field_name)
    };
    ($($tt:tt)+) => {
        select_field_attr!(@ACC [] $($tt)+)
    };
}
pub(crate) use select_field_attr;

#[cfg(test)]
mod tests {

    #[test]
    fn select_field_attr() {
        let h = select_field_attr!(hello);
        assert_eq!("hello", h);

        let goodbye = select_field_attr!(
            #[even_more_attributes]
            #[builder_field(rename = "goodbye")]
            #[even_more_attributes]
            hello
        );

        assert_eq!("goodbye", goodbye);
    }

    #[test]
    fn gen_field_names() {
        let single = gen_field_names!(
            #[even_more_attributes]
            #[builder_field(rename = "epic_name")]
            #[even_more_attributes]
            bad_name
        );

        assert_eq!(single, ["epic_name"]);

        let many = gen_field_names!(
            #[sasdsad]
            #[asdsadasdasd]
            asdasd,
            #[some_attr]
            #[my_attr]
            #[builder_field(rename = "cool_name")]
            terrible_field_name,
            #[even_more_attributes]
            #[builder_field(rename = "epic_name")]
            #[even_more_attributes]
            bad_name
        );

        assert_eq!(many, ["asdasd", "cool_name", "epic_name",]);
    }

    #[test]
    #[allow(unused)]
    fn masked_builder2() {
        masked_builder2!(
            struct A {
                pub asdasd: usize,
                pub b: usize,
            }
        );

        assert_eq!(A::FIELDS_SUM_LEN, 7);
        assert_eq!(A::FIELD_COUNT, 2);
        assert_eq!(A::FIELD_MAX_LEN, 6);
        assert_eq!(A::__FIELDS_SLICE, A::FIELDS.as_slice());
    }
}
