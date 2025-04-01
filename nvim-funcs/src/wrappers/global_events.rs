use nvim_types::{call_site::Channel, error::Error};
use thread_lock::call_check;

use crate::c_funcs;

pub fn nvim_error_event(err: Error) {
    call_check();
    unsafe {
        c_funcs::global_events::nvim_error_event(Channel::LUA_INTERNAL_CALL, err);
    }
}
