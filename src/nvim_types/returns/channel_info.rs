use crate::nvim_types::{Buffer, Channel, Dict, KVec, OwnedThinString};

use super::utils::skip_drop_remove_keys;

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
    pub id: Channel,
    // TODO: I am not fully sure if the returned values are static but they most likely arent.
    // To be safe assume they are not owned and just clone the values so worst case we have a
    // memory leak rather than UB
    pub argv: Option<KVec<OwnedThinString>>,
    pub stream: StreamKind,
    pub mode: ModeKind,
    // TODO: same as argv field
    pub pty: Option<OwnedThinString>,
    pub buffer: Option<Buffer>,
    // TODO: same as argv field
    pub client: Option<Dict>,
}

impl ChannelInfo {
    pub fn from_c_func_ret(d: &mut Dict) -> Self {
        let [id, argv, stream, mode, pty, buffer, client] = skip_drop_remove_keys(
            d,
            &["id", "argv", "stream", "mode", "pty", "buffer", "client"],
            None,
        )
        .unwrap();
        let id = Channel::new(id.as_int().unwrap());
        let argv = 'a: {
            let Some(argv) = argv.as_array() else {
                break 'a None;
            };
            let mut kv = KVec::with_capacity(argv.len());
            kv.extend(argv.iter().map(|obj| obj.as_string().unwrap().clone()));
            Some(kv)
        };
        let stream = match stream.as_string().unwrap().as_thinstr().as_slice() {
            b"stdio" => StreamKind::Stdio,
            b"stderr" => StreamKind::Stderr,
            b"socket" => StreamKind::Socket,
            b"job" => StreamKind::Job,
            s => unreachable!("Unknown variant for stream kind: {:?}", s),
        };
        let mode = match mode.as_string().unwrap().as_thinstr().as_slice() {
            b"bytes" => ModeKind::Bytes,
            b"terminal" => ModeKind::Terminal,
            b"rpc" => ModeKind::Rpc,
            s => unreachable!("Unknown variant for stream kind: {:?}", s),
        };
        let pty = pty.as_string().cloned();
        let buffer = buffer.as_buffer();
        let client = client.as_dict().cloned();

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
