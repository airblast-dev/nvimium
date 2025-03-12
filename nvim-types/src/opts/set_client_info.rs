use crate::string::ThinString;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ClientKind(ThinString<'static>);

impl ClientKind {
    pub const REMOTE: Self = Self(ThinString::from_null_terminated(b"remote\0"));
    pub const MSG_PACK_RPC: Self = Self(ThinString::from_null_terminated(b"msgpack-rpc\0"));
    pub const UI: Self = Self(ThinString::from_null_terminated(b"ui\0"));
    pub const EMBEDDER: Self = Self(ThinString::from_null_terminated(b"embedder\0"));
    pub const HOST: Self = Self(ThinString::from_null_terminated(b"host\0"));
    pub const PLUGIN: Self = Self(ThinString::from_null_terminated(b"plugin\0"));
}
