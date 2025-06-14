use crate::macros::tri::{tri_ez, tri_ret};
use crate::nvim_types::Arena;
use crate::nvim_types::object::ObjectRef;
use crate::nvim_types::returns::exec2::Exec2;
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
    let mut arena = Arena::EMPTY;
    tri_ret! {
        err;
        unsafe {
            vimscript::nvim_call_dict_function(
                ObjectRef::new_th(dict.as_thinstr()),
                func.as_thinstr(),
                args.into(),
                &raw mut arena,
                &mut err
            )
        };
        Object::clone;
    }
}

pub fn call_function<S: AsThinString>(func: S, args: &Array) -> Result<Object, Error> {
    call_check();
    let mut arena = Arena::EMPTY;
    tri_ret! {
        err;
        unsafe { vimscript::nvim_call_function(func.as_thinstr(), args.into(), &raw mut arena, &mut err) };
        Object::clone;
    }
}

pub fn command<S: AsThinString>(command: S) -> Result<(), Error> {
    call_check();
    tri_ez! {
        err;
        unsafe { vimscript::nvim_command(command.as_thinstr(), &mut err) };
    }
}

pub fn eval<S: AsThinString>(eval: S) -> Result<Object, Error> {
    call_check();

    let mut arena = Arena::EMPTY;
    tri_ret! {
        err;
        unsafe { vimscript::nvim_eval(eval.as_thinstr(), &raw mut arena, &mut err) };
        Object::clone;
    }
}

// TODO: replace dictionary with dedicated struct?
pub fn exec2<S: AsThinString>(exec: S, opts: &ExecOpts) -> Result<Exec2, Error> {
    // this functions is a bit of an odd one out
    //
    // everything is allocated and the function doesn't accept an arena
    // just handle this one manually
    call_check();
    let mut err = Error::none();
    let ret = unsafe {
        vimscript::nvim_exec2(
            Channel::LUA_INTERNAL_CALL,
            exec.as_thinstr(),
            opts,
            &mut err,
        )
    };
    if err.has_errored() {
        return Err(err);
    }

    let mut ret = unsafe { ret.assume_init() };
    Ok(Exec2::from_c_func_ret(&mut ret))
}

// TODO: create proper structs and what not.
// good amount of work to be done
fn parse_expression<S: AsThinString, S1: AsThinString>(
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
        Array, Buffer, Dict, NvString, Object, OwnedThinString,
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
                NvString::from("blue"),
                Object::String(OwnedThinString::from("#0000ff")),
            ),
            (
                NvString::from("red"),
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
