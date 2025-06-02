use crate::nvim_types::Boolean;

// we only provide this to avoid a breaking change when this opt gets added
//
// currently setting builtin to true will just result in an error anyways
#[repr(C)]
#[derive(Default)]
pub struct GetCommandOpts {
    builtin: Boolean,
}
