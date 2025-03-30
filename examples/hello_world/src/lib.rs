
use nvimium::{
    nvim_funcs::wrappers::global::{nvim_echo, nvim_get_hl_ns, nvim_set_hl},
    nvim_types::{func_types::echo::Echo, opts::{echo::EchoOpts, get_hl_ns::GetHlNsOpts}},
    plugin,
};

fn hello_world() {
    let echo = Echo::message(c"Example Error message!");
    let mut opts = EchoOpts::default();
    opts.err(true);
    nvim_echo(&echo, true, &opts).unwrap();
    nvim_echo(&Echo::message(c"Just everyday normal message."), true, &EchoOpts::default()).unwrap();
}

plugin!(luaopen_hello_world, hello_world);
