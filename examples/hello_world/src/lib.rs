use nvimium::{
    nvim_funcs::global::echo,
    nvim_types::{Integer, func_types::echo::Echo, opts::echo::EchoOpts},
    plugin,
};

fn hello_world() {
    let echo_msg = Echo::message(c"Example Error message!");
    let mut opts = EchoOpts::default();
    // once set neovim prints out the message with the error highlighting
    //
    // depending on the config the message will likely be displayed in red
    opts.err(true);
    // Both echo's have history as true so we can read it in testing.
    echo(&echo_msg, true, &opts).unwrap();
    echo(
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
    use nvimium::nvim_funcs::vimscript::exec2;
    use nvimium::{
        nvim_test,
        nvim_types::{dictionary::KeyValuePair, opts::exec::ExecOpts},
    };

    // The nvim_test macro spawns a neovim instance and loads this function in similar way to Lua's
    // `require`.
    #[nvim_test::nvim_test]
    fn hello_world() {
        // After calling this function the message log should contain the messages sent from our
        // function.
        super::hello_world();
        let mut result = exec2(c":messages", ExecOpts::default().output(true)).unwrap();
        let KeyValuePair { object, .. } = result.remove("output").unwrap();
        let output = object.into_string().unwrap();

        assert_eq!(
            output,
            "Example Error message!\nJust an everyday normal message."
        );
    }
}
