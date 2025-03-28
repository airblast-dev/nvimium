use std::{
    path::Path,
    process::{Command, Output},
};

#[cfg(feature = "testing")]
#[doc(hidden)]
pub use test_cdylib;

pub fn test_body(dylib_path: &Path, func_name: &str) -> Result<(), String> {
    // TODO: add better panic and error info
    let load_cmd = format!(
        ":lua package.loadlib([[{}]], '{func_name}')()",
        dylib_path.to_str().unwrap()
    );
    let mut cmd = Command::new("nvim");
    cmd.arg("--headless").arg("--clean").arg("-c").arg(load_cmd);
    let output = cmd.output();
    let o = match output {
        Ok(o) => o,
        Err(err) => return Err(err.to_string()),
    };
    let err = o.stderr;
    let err = String::from_utf8_lossy(&err);
    if !err.is_empty() {
        panic!("{err}");
    }

    Ok(())
}

fn match_output(o: &Output) -> Result<(), String> {
    if o.status.success() {
        Ok(())
    } else {
        let exit_msg = format!("Neovim exited with exit code: {}", o.status);
        Err(exit_msg)
    }
}

#[cfg(feature = "testing")]
#[macro_export]
macro_rules! test_pkg {
    () => {
        #[allow(unused)]
        #[cfg(test)]
        static CDYLIB_TEST_PATH: ::std::sync::LazyLock<::std::path::PathBuf> =
            ::std::sync::LazyLock::new($crate::test_cdylib::build_current_project);
    };
}

#[cfg(not(feature = "testing"))]
#[macro_export]
macro_rules! test_pkg {
    () => {};
}
