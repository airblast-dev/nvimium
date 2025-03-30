use std::mem::MaybeUninit;

use nvim_types::{
    Arena,
    array::Array,
    borrowed::Borrowed,
    call_site::Channel,
    dictionary::Dictionary,
    error::Error,
    object::{Object, ObjectRef},
    opts::echo::EchoOpts,
    string::ThinString,
};

unsafe extern "C" {
    pub fn nvim_call_dict_function<'a>(
        dict: ObjectRef<'a, ThinString<'a>>,
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
    pub fn nvim_command<'a>(command: ThinString<'a>, err: *mut Error);
    pub fn nvim_eval<'a>(eval: ThinString<'a>, err: *mut Error) -> MaybeUninit<Object>;
    pub fn nvim_exec2<'a>(
        chan: Channel,
        exec: ThinString<'a>,
        opts: *const EchoOpts,
        err: *mut Error,
    ) -> MaybeUninit<Dictionary>;
}
