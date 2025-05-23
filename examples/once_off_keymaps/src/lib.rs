use std::error::Error;
use nvimium::nvim_types::Error as NvError;

use nvimium::{
    nvim_funcs::global::{del_keymap, echo, set_keymap},
    nvim_types::{
        func_types::{echo::Echo, keymap_mode::KeyMapMode},
        opts::{echo::EchoOpts, set_keymap::SetKeymapOpts},
    },
    plugin,
};

fn once_off_keymaps() -> Result<(), Box<dyn Error>> {
    set_keymap(
        KeyMapMode::INSERT,
        c"1",
        c"",
        // The error type must be specified to use the "?" operator
        SetKeymapOpts::default().callback::<NvError>(move |_| {
            echo(
                &Echo::message("Pressed 1\nShould be called once."),
                true,
                EchoOpts::default().err(true),
            )?;
            del_keymap(KeyMapMode::INSERT, c"1")?;
            Ok(())
        }),
    )?;
    Ok(())
}

// create our lua entrypoint
plugin!(luaopen_once_off_keymaps, once_off_keymaps);

#[cfg(feature = "testing")]
mod tests {
    // testing keymaps can get very complicated, instead you are encouraged to test a function that
    // acts upon a keymap seperately
}
