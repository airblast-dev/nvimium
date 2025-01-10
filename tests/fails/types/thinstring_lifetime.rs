use nvim_types::string::{OwnedThinString, String, AsThinString};

fn string(s: String) {
    let th = s.as_thinstr();
    drop(s);
    dbg!(th);
}

fn owned(ow: OwnedThinString) {
    let th = ow.as_thinstr();
    drop(ow);
    dbg!(th);
}

fn cstring(cs: std::ffi::CString) {
    let th = cs.as_thinstr();
    drop(cs);
    dbg!(th);
}

fn cstr(cs: std::ffi::CString) {
    let cstr = cs.as_c_str();
    let th = cstr.as_thinstr();
    drop(cs);
    dbg!(th);
}
