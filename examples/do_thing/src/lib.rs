use std::error::Error;

use nvimium::{
    nvim_funcs::global::{echo, set_keymap},
    nvim_types::{
        func_types::{echo::Echo, keymap_mode::KeyMapMode},
        opts::{echo::EchoOpts, set_keymap::SetKeymapOpts},
    },
    plugin,
};

fn do_thing() -> Result<(), Box<dyn Error>> {
    set_keymap(
        KeyMapMode::INSERT,
        c"1",
        c"",
        SetKeymapOpts::default().callback(|_| {
            let _ = echo(
                &Echo::message("Pressed 1"),
                true,
                EchoOpts::default().err(true),
            );
        }),
    )?;
    Ok(())
}

// create our lua entrypoint
plugin!(luaopen_do_thing, do_thing);

#[cfg(feature = "testing")]
mod tests {
    // testing keymaps can get very complicated, instead you are encouraged to test a function that
    // acts upon a keymap seperately
}
