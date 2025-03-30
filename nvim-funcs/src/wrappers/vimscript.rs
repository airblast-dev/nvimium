use nvim_types::{
    array::Array, call_site::Channel, dictionary::Dictionary, error::Error, object::Object,
    opts::echo::EchoOpts, string::AsThinString,
};
use thread_lock::call_check;

use crate::c_funcs::vimscript;

use macros::tri;

pub fn nvim_call_dict_function<S1: AsThinString, S2: AsThinString>(
    dict: S1,
    func: S2,
    args: &Array,
) -> Result<Object, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe {
            vimscript::nvim_call_dict_function(
                (&dict.as_thinstr()).into(),
                func.as_thinstr(),
                args.into(),
                core::ptr::null_mut(),
                &mut err
            )
        },
        Ok(obj) => Ok(unsafe { obj.assume_init() })
    }
}

pub fn nvim_call_function<S: AsThinString>(func: S, args: &Array) -> Result<Object, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { vimscript::nvim_call_function(func.as_thinstr(), args.into(), core::ptr::null_mut(), &mut err) },
        Ok(obj) => Ok(unsafe{ obj.assume_init() })
    }
}

pub fn nvim_command<S: AsThinString>(command: S) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { vimscript::nvim_command(command.as_thinstr(), &mut err) }
    }
}

pub fn nvim_eval<S: AsThinString>(eval: S) -> Result<Object, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { vimscript::nvim_eval(eval.as_thinstr(), &mut err) },
        Ok(obj) => Ok(unsafe { obj.assume_init() })
    }
}

// TODO: replace dictionary with dedicated struct?
pub fn nvim_exec2<S: AsThinString>(exec: S, opts: &EchoOpts) -> Result<Dictionary, Error> {
    tri! {
        let mut err;
        unsafe{ vimscript::nvim_exec2(Channel::LUA_INTERNAL_CALL, exec.as_thinstr(), opts, &mut err) },
        Ok(d) => Ok(unsafe{ d.assume_init() })
    }
}
