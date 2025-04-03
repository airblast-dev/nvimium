// The test system works by creating a tempfile and then sending its path to neovim via
// a global variable.
//
// This variable is then read by our plugin and any panic is recorded to the temp file.
//
// Once the command is finished we read the file to see if there has been a panic.
//
// # Note
//
// A test file is required in order to test some functions as they write to `stderr`. Instead we
// are required to replace the default panic hook with our own where the hook writes the panic info
// to file that is unrelated to neovim.
use std::{backtrace::Backtrace, fs::File, io::Write, panic::set_hook, path::PathBuf};

#[cfg(feature = "testing")]
mod testing_imports {
    pub use std::path::Path;
    pub use std::process::Command;
}

#[cfg(feature = "testing")]
use testing_imports::*;

#[cfg(feature = "testing")]
pub use nvim_test_macro::*;
pub use thread_lock;

#[cfg(feature = "testing")]
#[doc(hidden)]
pub use test_cdylib;

// track_caller gives us a better line col value
// in a panic message it allows to be read as at function xy, a panic at xy occured
#[track_caller]
#[cfg(feature = "testing")]
pub fn test_body(dylib_path: &Path, func_name: &str) -> Result<(), String> {
    use std::{io::Read, process::Stdio};
    use tempfile::NamedTempFile;
    let load_cmd = format!(
        ":lua package.loadlib([[{}]], '{func_name}')()",
        dylib_path.to_str().unwrap()
    );
    let mut cmd = Command::new("nvim");
    // not required but some tests end up writing to stderr without an actual error so just pass a
    // pipe
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut err_file = NamedTempFile::new().unwrap();
    cmd.arg("--headless")
        .arg("--clean")
        .arg("-c")
        .arg(load_cmd)
        .arg("--cmd")
        // pass our fake stderr file path to neovim so we can later retreive it in the cdylib
        .arg(format!(
            ":lua vim.api.nvim_set_var([[NVIMIUM_PANIC_LOG_FILE]], [[{}]])",
            err_file.path().to_str().unwrap()
        ));
    let status = cmd.status().unwrap();

    let mut panic_out = String::new();
    err_file.read_to_string(&mut panic_out).unwrap();

    // the file should only contain something if there was a panic
    if !panic_out.is_empty() {
        panic!("{}", panic_out);
    }

    assert!(status.success());

    Ok(())
}

/// Builds and stores a path to the test cdylib
///
/// Internally uses [`std::sync::LazyLock`] to reuse the same built binary and only initialize it
/// if needed.
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

/// Testing nvimiums "testing" feature is disabled
///
/// Enable it to use this macro
#[cfg(not(feature = "testing"))]
#[macro_export]
macro_rules! test_pkg {
    () => {};
}

/// Sets the panic hook to write to the provided path
///
/// This is only used when testing a cdylib since some tests may write to stderr.
#[track_caller]
pub fn set_test_panic_hook(p: PathBuf) {
    set_hook(Box::new(move |pi| {
        let mut err_file = File::options().append(true).open(&p).unwrap();
        writeln!(err_file, "{}", pi).unwrap();
        writeln!(err_file, "{}", Backtrace::force_capture()).unwrap();
        err_file.flush().unwrap();
    }));
}
