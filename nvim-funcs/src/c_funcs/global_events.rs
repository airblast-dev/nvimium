use nvim_types::{call_site::Channel, error::Error};

unsafe extern "C" {
    pub fn nvim_error_event(chan: Channel, err: Error);
}

#[cfg(feature = "testing")]
mod testing {
    use nvim_types::call_site::Channel;
    use nvim_types::error::Error;
    use nvim_types::string::{String, ThinString};
    use thread_lock::unlock;

    use crate::wrappers::vimscript::nvim_exec2;

    // we actually cant test if this succedes but if the test fails or hangs we can tell something
    // is wrong
    #[nvim_test_macro::nvim_test(exit_call = nvim_exec2)]
    fn test_nvim_error_event() {
        let err = Error::validation(ThinString::from_null_terminated(b"Hello World\0"));

        unsafe { super::nvim_error_event(Channel::LUA_INTERNAL_CALL, err) };
    }
}
