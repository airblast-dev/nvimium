use nvim_types::{call_site::Channel, error::Error};

unsafe extern "C" {
    pub fn nvim_error_event(chan: Channel, err: Error);
}

#[cfg(feature = "testing")]
mod testing {
    use crate as nvim_funcs;
    use nvim_types::call_site::Channel;
    use nvim_types::error::Error;
    use nvim_types::string::ThinString;

    // we actually cant test if this succedes but if the test fails or hangs we can tell something
    // is wrong
    #[nvim_test::nvim_test]
    fn test_nvim_error_event() {
        let err = Error::validation(ThinString::from_null_terminated(b"Hello World\0"));

        unsafe { super::nvim_error_event(Channel::LUA_INTERNAL_CALL, err) };
    }
}
