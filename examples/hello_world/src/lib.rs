use nvimium::{
    nvim_funcs::global::nvim_echo,
    nvim_types::{func_types::echo::Echo, opts::echo::EchoOpts},
    plugin,
};

fn hello_world() {
    let echo = Echo::message(c"Example Error message!");
    let mut opts = EchoOpts::default();
    // once set neovim prints out the message with the error highlighting
    //
    // depending on the config the message will likely be displayed in red
    opts.err(true);
    // Both echo's have history as true so we can read it in testing.
    nvim_echo(&echo, true, &opts).unwrap();
    nvim_echo(
        &Echo::message(c"Just an everyday normal message."),
        true,
        &EchoOpts::default(),
    )
    .unwrap();
}

// create our lua entrypoint
plugin!(luaopen_hello_world, hello_world);

#[cfg(feature = "testing")]
mod tests {
    use nvimium::nvim_funcs;
    use nvimium::nvim_funcs::vimscript::nvim_exec2;
    use nvimium::{
        nvim_test,
        nvim_types::{dictionary::KeyValuePair, opts::exec::ExecOpts, string::AsThinString},
    };

    /// The nvim_test macro requires that `nvim_funcs` and `nvim_tests` are in scope!
    #[nvim_test::nvim_test]
    fn hello_world() {
        super::hello_world();
        let mut result = nvim_exec2(c":messages", ExecOpts::default().output(true)).unwrap();
        let KeyValuePair { object, .. } = result.remove(c"output".as_thinstr()).unwrap();
        let output = object.into_string().unwrap();

        assert_eq!(
            output,
            "Example Error message!\nJust an everyday normal message."
        );
    }
}
