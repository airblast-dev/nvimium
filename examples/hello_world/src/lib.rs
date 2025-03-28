use nvimium::{
    nvim_funcs::wrappers::global::nvim_set_keymap,
    nvim_types::{
        func_types::keymap_mode::KeyMapMode, opts::set_keymap::SetKeymapOpts, string::AsThinString,
    },
    plugin,
};

fn hello_world() {
    let mut opts = SetKeymapOpts::default();
    opts.desc(c"HELLOOOOO");
    nvim_set_keymap(
        KeyMapMode::VISUAL,
        c"a".as_thinstr(),
        c"b".as_thinstr(),
        &opts,
    )
    .unwrap();
}

plugin!(luaopen_hello_world, hello_world);
