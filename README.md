# What is Nvimium?

Nvimium is Rust library that provides a safe interface to Neovim's C bindings. 

# Quickstart

Creating a plugin is really simple and consists of two main steps:
- Create a function to run when loading our plugin.
- Register the plugin via the `plugin!` macro.


Here is a very simple example that prints a few messages in Neovim.
```rust
use std::error::Error;

use nvimium::{
    nvim_funcs::global::echo,
    nvim_types::{func_types::echo::Echo, opts::echo::EchoOpts},
    plugin,
};

fn hello_world() -> Result<(), Box<dyn Error>> {
    let echo_msg = Echo::message(c"Example Error message!");
    let mut opts = EchoOpts::default();
    // once set neovim prints out the message with the error highlighting
    //
    // depending on the config the message will likely be displayed in red
    opts.err(true);
    echo(&echo_msg, true, &opts)?;
    echo(
        &Echo::message(c"Just an everyday normal message."),
        true,
        &EchoOpts::default(),
    )?;

    Ok(())
}

// create our lua entrypoint
plugin!(luaopen_hello_world, hello_world);
```

# Problems Nvimium Solves

## Complexity

Plugins written in C that leverage the C API generally require deep knowledge of Neovim as each function it provides contains a slightly different deallocation scheme.
Instead Nvimium is able to express complex mechanisms by providing safe wrappers allowing you to focus on what your plugin is intended to do.

## Performance

When writing a plugin you may want to perform some expensive computation, but Lua isn't the fastest so you might want to try a more performant language.
This is where Nvimium steps in and provides a safe interface where you can write your expensive code without worrying about safety. 
Currently most plugins with such requirements are written with raw C bindings, but Nvimium aims to provide a safe abstraction over the raw API by 
leveraging Rust's strong type system.

## Safety

Nvimium provides safety via allowing and blocking access to Neovim and Lua functions where they shouldn't be called.
Neovim internally uses mutable statics in many parts of its codebase which makes nearly all of its functions a thread safety hazard.
To avoid race conditions and many other nasty problems Nvimium makes use of thread locals that track if we are allowed to call a 
function in the current context (such as a callbacks, entrypoint, ...).

While this enables all of the wrapper functions to be thread safe it does block you from doing a few things:
- Calling a Neovim function from another thread that wasn't yielded execution by Neovim.
- Passing around variables that are thread, or Lua state specific such as Lua references.

You are technically allowed to call a Neovim function from another thread, but it will result in panic unless you acquire access through that thread 
or make use of other workarounds (highly unsafe!!!).

In case you are willing to squeeze some performance out of expensive functions at the cost of compile time guarantees, 
the C bindings are also exported through this library. 
You are genereally discouraged to use these as some functions have odd deallocation strategies so only use if them if you are absolutely 
sure you need it and can guarantee their safety. These functions are also exempt of any safety and semver guarantee's that this library provides.

Ofcourse there are many other small safety guards in place but this is intended to just be a short summary of how safety is provided over the C API.

## Compile Times

Its generally not feasible to distribute a binary for every single target out there.
While binaries can be provided for common targets it still doesn't solve the main problem at hand.

To help this problem Nvimium costs very little in compile time and avoids use of any proc-macros (and dependencies that use proc-macros) allowing 
a plugin user to compile or update a plugin fairly quickly as long as they have cargo installed.

## Binary Size

Rust is notorious for its large binary sizes. To avoid huge plugins Nvimium attempts to use as little space as possible in your binary
by avoiding use of generics and leveraging dynamic dispatch where its performance cost is irrelevant (such as printing an `Error` in a callback that returns an error).

However though the result still is pretty large binaries but this at least allows you to shrink it further by using various experimental compiler flags.

## Out of Memory Recovery

Nvimium provides and uses a wrapper for the `System` allocator from the standard library called `NvAllocator` that is able to request that
Neovim free's unused memory blocks and collects some garbage if an allocation fails due to memory exhaustion. 
This is only performed if the current thread is one that has execution yielded to it, such as inside a callback.

`NvAllocator` also implements the `GlobalAllocator` trait from the standard library allowing you to set it as your global allocator in case you also 
want to leverage the same mechanism in allocating types from other libraries (such as `Vec`, `Box`, ...).

It is recommended to use this as a gobal allocator in plugins as it will allow recovering from a hard crash.

## Backwards Compatibility

Neovim often makes changes to the C API on major version changes making it hard to provide backwards Compatibility in a safe way 
which results in breaking changes for Nvimium as well.
Since not all plugins will want to support the latest Neovim version out there, the library offers and maintains support for the last two major versions of Neovim starting from `0.11`.

This also means that writing a plugin that supports a bunch of Neovim versions will be very cumbersome when using Nvimium, or any other library that provides wrappers for the C bindings.

## Testing

Rust has a great testing framework but it is lacking quite a bit when it comes to testing `cdylib` crates. 
Nvimium provides macros and functions that aid in testing your plugin.

For a quick and small example check out the `hello_world` example and its test.

# Getting Started

## The Book
TODO: add book

## Examples

The `examples` directory contains a few plugins that use Nvimium. Most of these plugins are intended to be very simple making them suitable for beginners.

Check out `examples/README.md` for more information.
