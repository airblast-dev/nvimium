use nvim_types::{Integer, call_site::Channel, string::ThinString};

unsafe extern "C" {
    pub fn nvim_error_event<'a>(chan: Channel, error_type: Integer, message: ThinString<'a>);
}

#[cfg(feature = "testing")]
mod testing {
    use nvim_types::call_site::Channel;
    use nvim_types::string::String;

    use crate::wrappers::global::nvim_exec;

    // we actually cant test if this succedes but if the test fails or hangs we can tell something
    // is wrong
    #[nvim_test_macro::nvim_test(exit_call = nvim_exec)]
    fn test_nvim_error_event() {
        use nvim_types::string::AsThinString;

        unsafe { super::nvim_error_event(Channel::LUA_INTERNAL_CALL, 1, c"Hello".as_thinstr()) };
    }
}
