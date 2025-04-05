# Contributing Guidelines

Pull Requests, bug reports and any other form of contribution is highly appreciated.

## Opening an Issue
If you think you have encountered a bug please search the existing issues before creating an issue.

## Feature Requests
If you want a new feature in this library its a good idea to create an issue describing the feature.

Not all features will be implemented such as Neovim's Lua API as this library focuses on the C API.
Only exceptions to this rule is where using the C API basically requires Lua interaction.

Features outside of Neovim's stable API will likely not be implemented, the exception to this rule is 
a few recovery functions in case a panic or an allocation fails as they allow us to lower the chance
of data loss and possibly fully recover from said panic or failed allocation.

## Code Guidelines

### Code Style
The code is checked with `clippy` and formatted with `cargo fmt`.
Public functions must be documented and easy to use as they should not feel like the user is 
interacting with a C API.
Internal functions do not have this requirment.

### Compile Time and Dependencies
Nvimium intentionally avoids heavy dependencies to be able to provide fast compile times.
This is done to simplify distrubution for downstream projects as they may not want to distribute binaries
for every platform they support. Though they are still encouraged to distrubute binaries for 
all platforms that Neovim officially supports.

In the case a large dependency or piece of code that heavily affects compile times is required, such 
functionality should be behind a feature gate. This does not apply to test related functions and macros as they 
are already feature gated, only to be used in tests.

### Binary Size
While it is impossible to match the size of a Lua script, bloating the binary should still be 
avoided where possible.

Functions provided by `libc` should be preferred if Neovim is also making the assumption of a 
symbol existing for a system or the symbol exists on practically every platform.

Internal functions should avoid generics where possible for better compile times and binary size.

### Performance
Performance isn't a top priority but should be reasonably fast. Improving the performance is 
always a win as long as it doesn't effect complexity too much.

### Unsafe Usage
Since we are dealing with Neovim's C API usage of unsafe is required. Unsafe code should be
encapsulated and tested with `miri` where possible. 
In cases where a symbol isn't supported by `miri`, integration tests should be written to confirm behavior.
Some tests require `miri` feature gates and are ran with and without `miri`.
