use crate::nvim_types::{
    Integer, Array, Buffer, Dict, Object,
    string::OwnedThinString,
};

#[derive(Clone, Copy, Debug)]
pub enum StreamKind {
    Stdio,
    Stderr,
    Socket,
    Job,
}

#[derive(Clone, Copy, Debug)]
pub enum ModeKind {
    Bytes,
    Terminal,
    Rpc,
}

pub struct ChannelInfo {
    pub id: Integer,
    // TODO: I am not fully sure if the returned values are static but they most likely arent.
    // To be safe assume they are not owned and just clone the values so worst case we have a
    // memory leak rather than UB
    pub argv: Array,
    pub stream: StreamKind,
    pub mode: ModeKind,
    // TODO: same as argv field
    pub pty: Option<OwnedThinString>,
    pub buffer: Option<Buffer>,
    // TODO: same as argv field
    pub client: Option<Dict>,
}

impl ChannelInfo {
    pub fn from_c_func_ret(d: &Dict) -> Self {
        let id = d
            .get("id")
            .cloned()
            .map(|obj| obj.into_int().unwrap())
            .unwrap();
        let argv = d
            .get("argv")
            .cloned()
            .map(|obj| obj.into_array().unwrap())
            .unwrap();
        let stream = match d.get("stream").unwrap() {
            Object::String(s) => match s.as_thinstr().as_slice() {
                b"stdio" => StreamKind::Stdio,
                b"stderr" => StreamKind::Stderr,
                b"socket" => StreamKind::Socket,
                b"job" => StreamKind::Job,
                s => unreachable!("Unknown variant for stream kind: {:?}", s),
            },
            s => unreachable!("Unknown object type for stream field: {:?}", s),
        };
        let mode = match d
            .get("mode")
            .expect("mode not found in channel info dictionary")
        {
            Object::String(mode) => match mode.as_thinstr().as_slice() {
                b"bytes" => ModeKind::Bytes,
                b"terminal" => ModeKind::Terminal,
                b"rpc" => ModeKind::Rpc,

                s => unreachable!("Unknown variant for stream kind: {:?}", s),
            },
            s => unreachable!("Unknown object type for mode kind: {:?}", s),
        };

        let pty = d
            .get("pty")
            .cloned()
            .map(|obj| obj.into_string().unwrap())
            .clone();
        let buffer = d
            .get("buffer")
            .cloned()
            .map(|obj| obj.into_buffer().unwrap());
        let client = d.get("client").cloned().map(|obj| obj.into_dict().unwrap());

        Self {
            id,
            argv,
            stream,
            mode,
            pty,
            buffer,
            client,
        }
    }
}
