use std::mem::MaybeUninit;

use nvim_types::{
    Arena,
    array::Array,
    borrowed::Borrowed,
    error::Error,
    object::{Object, ObjectRef},
    string::ThinString,
};

unsafe extern "C" {
    pub fn nvim_call_dict_function<'a>(
        dict: ObjectRef<'a ,ThinString<'a>>,
        func: ThinString<'a>,
        args: Borrowed<'a, Array>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;

    pub fn nvim_call_function<'a>(
        func: ThinString<'a>,
        args: Borrowed<'a, Array>,
        arena: *mut Arena,
        err: *mut Error,
    ) -> MaybeUninit<Object>;
}
