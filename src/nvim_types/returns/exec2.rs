use crate::nvim_types::{Dict, OwnedThinString};

#[derive(Clone, Debug)]
pub struct Exec2 {
    pub output: Option<OwnedThinString>,
}

impl Exec2 {
    pub(crate) fn from_c_func_ret(d: &mut Dict) -> Self {
        let output = d.remove(c"output").and_then(|o| o.object.into_string());
        Self { output }
    }
}
