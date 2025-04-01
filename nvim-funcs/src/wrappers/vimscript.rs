use nvim_types::{
    array::Array, call_site::Channel, dictionary::Dictionary, error::Error, object::Object, opts::{echo::EchoOpts, exec::ExecOpts}, string::AsThinString, Boolean
};
use thread_lock::call_check;

use crate::c_funcs::vimscript;

use macros::tri;

// TODO: add test
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
        unsafe { vimscript::nvim_eval(eval.as_thinstr(), core::ptr::null_mut(), &mut err) },
        Ok(obj) => Ok(unsafe { obj.assume_init() })
    }
}

// TODO: replace dictionary with dedicated struct?
pub fn nvim_exec2<S: AsThinString>(exec: S, opts: &ExecOpts) -> Result<Dictionary, Error> {
    tri! {
        let mut err;
        unsafe{ vimscript::nvim_exec2(Channel::LUA_INTERNAL_CALL, exec.as_thinstr(), opts, &mut err) },
        // uses PUT (allocating conversion) for the key string
        Ok(d) => Ok(unsafe{ d.assume_init() })
    }
}

pub fn nvim_parse_expression<S: AsThinString, S1: AsThinString>(
    eval: S,
    flags: S1,
    highlight: Boolean,
) -> Result<Dictionary, Error> {
    // TODO: likely a memory leak, replace with dedicated struct
    tri! {
        let mut err;
        unsafe { vimscript::nvim_parse_expression(eval.as_thinstr(), flags.as_thinstr(), highlight, core::ptr::null_mut(), &mut err) },
        Ok(d) => Ok(unsafe { d.assume_init_ref() }.clone())
    }
}

#[cfg(feature = "testing")]
mod tests {
    use crate::wrappers::global::{nvim_feedkeys, nvim_list_bufs, nvim_set_current_buf};

    use super::nvim_exec2;
    use nvim_types::{
        array::Array,
        buffer::Buffer,
        dictionary::Dictionary,
        func_types::feedkeys::{FeedKeysMode, FeedKeysModeKind},
        kvec::KVec,
        object::Object,
        string::{OwnedThinString, String},
    };
    use thread_lock::unlock;

    #[nvim_test_macro::nvim_test(exit_call = nvim_exec2)]
    pub fn nvim_call_function() {
        nvim_set_current_buf(Buffer::new(1)).unwrap();
        nvim_feedkeys(
            c"iHello\nabc\nBye",
            &FeedKeysMode::from([FeedKeysModeKind::Typed]),
            false,
        );
        let res = super::nvim_call_function(
            c"string",
            &Array(KVec::from_iter([Object::Array(Array(KVec::from_iter(
                (0..20).map(Object::Integer),
            )))])),
        )
        .unwrap()
        .into_string()
        .unwrap();

        assert_eq!(
            res,
            "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]"
        );
    }

    #[nvim_test_macro::nvim_test(exit_call = nvim_exec2)]
    pub fn nvim_command() {
        let arr = nvim_list_bufs();
        assert_eq!(arr.len(), 1);
        super::nvim_command(c"new").unwrap();
        let arr = nvim_list_bufs();
        assert_eq!(arr.len(), 2);
    }

    #[nvim_test_macro::nvim_test(exit_call = nvim_exec2)]
    pub fn nvim_eval() {
        let expr = cr###"#{blue: "#0000ff", red: "#ff0000"}"###;
        let res = super::nvim_eval(expr).unwrap().into_dict().unwrap();
        let expected = Dictionary::from_iter([
            (
                String::from("blue"),
                Object::String(OwnedThinString::from("#0000ff")),
            ),
            (
                String::from("red"),
                Object::String(OwnedThinString::from("#ff0000")),
            ),
        ]);
        assert_eq!(res, expected);
    }
}
