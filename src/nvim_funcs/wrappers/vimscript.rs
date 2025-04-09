use crate::nvim_types::{
    Array, AsThinString, Boolean, Channel, Dict, Error, Object, opts::exec::ExecOpts,
};
use thread_lock::call_check;

use crate::nvim_funcs::c_funcs::vimscript;

use crate::tri;

// TODO: add test
pub fn call_dict_function<S1: AsThinString, S2: AsThinString>(
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

pub fn call_function<S: AsThinString>(func: S, args: &Array) -> Result<Object, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { vimscript::nvim_call_function(func.as_thinstr(), args.into(), core::ptr::null_mut(), &mut err) },
        Ok(obj) => Ok(unsafe{ obj.assume_init() })
    }
}

pub fn command<S: AsThinString>(command: S) -> Result<(), Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { vimscript::nvim_command(command.as_thinstr(), &mut err) }
    }
}

pub fn eval<S: AsThinString>(eval: S) -> Result<Object, Error> {
    call_check();
    tri! {
        let mut err;
        unsafe { vimscript::nvim_eval(eval.as_thinstr(), core::ptr::null_mut(), &mut err) },
        Ok(obj) => Ok(unsafe { obj.assume_init() })
    }
}

// TODO: replace dictionary with dedicated struct?
pub fn exec2<S: AsThinString>(exec: S, opts: &ExecOpts) -> Result<Dict, Error> {
    tri! {
        let mut err;
        unsafe{ vimscript::nvim_exec2(Channel::LUA_INTERNAL_CALL, exec.as_thinstr(), opts, &mut err) },
        // uses PUT (allocating conversion) for the key string
        Ok(d) => Ok(unsafe{ d.assume_init() })
    }
}

pub fn parse_expression<S: AsThinString, S1: AsThinString>(
    eval: S,
    flags: S1,
    highlight: Boolean,
) -> Result<Dict, Error> {
    // TODO: likely a memory leak, replace with dedicated struct
    // it seems that the returned Dict contains a mix of owned and static strings which makes this
    // pretty hard to expose to users
    //
    // this is fine for now but should eventually use a dedicated struct in the return type
    tri! {
        let mut err;
        unsafe { vimscript::nvim_parse_expression(eval.as_thinstr(), flags.as_thinstr(), highlight, core::ptr::null_mut(), &mut err) },
        Ok(d) => Ok(unsafe { d.assume_init_ref() }.clone())
    }
}

#[cfg(all(not(miri), feature = "testing"))]
mod tests {
    use crate as nvimium;
    use crate::nvim_funcs::wrappers::global::{feedkeys, list_bufs, set_current_buf};

    use crate::nvim_types::{
        Array, Buffer, Dict, Object, OwnedThinString, String,
        func_types::feedkeys::{FeedKeysMode, FeedKeysModeKind},
        kvec::KVec,
    };

    #[nvim_test::nvim_test]
    pub fn nvim_call_function() {
        set_current_buf(Buffer::new(1)).unwrap();
        feedkeys(
            c"iHello\nabc\nBye",
            &FeedKeysMode::from([FeedKeysModeKind::Typed]),
            false,
        );
        let res = super::call_function(
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

    #[nvim_test::nvim_test]
    pub fn nvim_command() {
        let arr = list_bufs();
        assert_eq!(arr.len(), 1);
        super::command(c"new").unwrap();
        let arr = list_bufs();
        assert_eq!(arr.len(), 2);
    }

    #[nvim_test::nvim_test]
    pub fn nvim_eval() {
        let expr = cr###"#{blue: "#0000ff", red: "#ff0000"}"###;
        let res = super::eval(expr).unwrap().into_dict().unwrap();
        let expected = Dict::from_iter([
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

    #[nvim_test::nvim_test]
    pub fn nvim_parse_expression() {
        // TODO: add proper testing to ensure that we get the expected result
        let expr = c"echo  123";
        super::parse_expression(expr, c"", false).unwrap();
    }
}
