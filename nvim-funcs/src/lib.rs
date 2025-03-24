use std::{path::PathBuf, sync::LazyLock};

pub(crate) static CDYLIB_TEST_PATH: LazyLock<PathBuf> =
    LazyLock::new(test_cdylib::build_current_project);

pub mod c_funcs;
pub mod wrappers;
