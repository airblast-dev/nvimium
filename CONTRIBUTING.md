# Thanks!

First of all, thanks for considering contributing to the project.

The following sections will get you up to speed before diving into the code base.

# Testing

The library contains a ton of unsafe which is required in order to interact with a C API.
Most FFI related libraries cannot be tested using `miri` and this is also true for Nvimium.

However some of the code that uses C bindings is compatible with `miri` so such functions are tested with `miri` wherever possible.

## Running the Tests

Nvimium uses a tool called [cargo-make](https://github.com/sagiegurari/cargo-make) in order to instrument the tests.

Running the command `cargo make test` will run all of the tests with and without `miri`. This command will also spawn Neovim instances that test marked functions. 

Tests that require Neovim's C functions to be exported will be loaded by Neovim where the whole project is compiled as a `cdylib` and then imported by Neovim instances.
After that each test will run and report if the tests has passed or failed with information on why a test failed.

# Features

Generally features exposed in this library are dependent of which features are provided by Neovim.

Any and all features should eventually make it in Nvimium where a safe API is possible.
If a feature is impossible to implement safely, it should be implemented in a seperate crate.

Features should be proposed in an Issue before working on a PR.

TODO: create a new crate containing functions that are impossible to make safe

# Fixes

Any fixes and safety improvement are always welcome.

Before creating a PR please create an Issue describing the bug, this makes finding information related to a bug easier to find compared than searching through pull requests.

Such PR's should generally contain a test to ensure that a guaranteed or required behavior is upheld.

# Breaking Changes

Generally breaking changes should be avoided between minor Neovim releases, but this isn't a strict requirement.

However since Nvimium is in its early stages, breaking changes are expected to be frequent 
so don't think about it too hard until the library gets in a more stable state.

# Scope

Nvimium aims to only provide safe wrappers for Neovim's C API. Neovim's Lua functions are not intended to be supported in this library.

In some cases it may be required to use the Lua API in order provide some basic functionality, or it may be useful during testing. Such functions 
are exempt from the rules above and can be added to Nvimium.

TODO: write wrappers for neovims lua functions in a seperate crate and link it here

# Notes

The sections above should be enough to get you started, but there is a few things to keep in mind.

Aim to write safe and reasonably fast code. 

Try to not bloat the binary size if other solutions exist.

Avoid adding dependencies if possible. Each dependency will likely hurt compile times and the binary size of plugins.
That said, some dependencies are a neccesity if we want to avoid a maintenance overhead.
