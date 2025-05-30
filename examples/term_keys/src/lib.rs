use nvimium::nvim_types::Error as NvError;
use std::error::Error;

use nvimium::{
    nvim_funcs::global::{create_buf, echo, open_term, set_current_buf},
    nvim_types::{
        func_types::echo::Echo,
        opts::{echo::EchoOpts, open_term::OpenTermOpts},
    },
    plugin,
};

fn term_keys() -> Result<(), Box<dyn Error>> {
    let buf = create_buf(true, false)?;
    open_term(
        buf,
        // add a callback that prints the key pressed in this terminal
        OpenTermOpts::default().on_input::<NvError>(|args| {
            echo(&Echo::message(args.data), true, &EchoOpts::default())
        }),
    )?;
    set_current_buf(buf)?;
    Ok(())
}

// create our lua entrypoint
plugin!(luaopen_term_keys, term_keys);

#[cfg(feature = "testing")]
mod tests {
    // testing keymaps can get very complicated, instead you are encouraged to test a function that
    // acts upon a keymap seperately
}
