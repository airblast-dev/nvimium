// This works exactly how I want it to work but not a fan of the possible cost
// TODO: no idea how this would work with LUV

//! Various sync utilities for Neovim and Lua functions
//!
//! The module contains a thread local so callers can request access to Neovim functions.
//! Calling Neovim functions from other threads require an explicit call to functions located in
//! the module.
//!
//! Thread safety is provided via three steps:
//! - Every function called by neovim checks if it is allowed access.
//! - All functions called must be wrapped with a scope (often with [`scoped`]) that catches panics and sets/revokes the
//!   threads access.
//! - The main Lua pointer is checked if it is null. This is done to avoid version mismatches with
//!   dependencies that use a different version of nvimium as the static variable are shared with the
//!   same versions of a crate.
use std::{
    cell::Cell,
    marker::PhantomData,
    mem::ManuallyDrop,
    panic::{AssertUnwindSafe, catch_unwind, resume_unwind},
    ptr::NonNull,
    sync::atomic::{AtomicPtr, Ordering},
};

use mlua_sys::lua_State;

thread_local! {
    /// When true, the current thread was yielded execution and is allowed to perform calls to neovim.
    static HAS_ACCESS: Cell<bool> = const { Cell::new(false) }
}

#[derive(Default)]
pub struct ThLock(PhantomData<*mut u8>);

/// Calling this function gives access to the current thread to call a neovim function
///
/// If no OS threads are spawned via [`std::thread::spawn`] or other means this function does not
/// need to be called as nvimium will handle the locking for callbacks and entrypoints.
///
/// There is only two reasons to use this:
/// - A callback or entrypoint spawns another thread where only one thread calls Neovim functions
/// - Neovim functions are called from multiple threads but other synchronization is achieved by
///   other means such as a [`std::sync::Mutex`] or atomics.
///
/// Regardless of the reason, you are highly discouraged from calling this as in all cases nvimium
/// should call it where it is safe.
///
/// # Safety
///
/// Neovim uses mutable statics in many parts of its codebase, this means multiple functions
/// should never be called at once. After calling this function multiple threads must not call
/// neovim or lua functions at once. Not doing so may result in UB depending on the functions being
/// called.
pub unsafe fn unlock() -> ThLock {
    HAS_ACCESS.set(true);

    ThLock(PhantomData)
}

/// Revokes access from the current thread
// we currently don't have to pass ThLock, only done to avoid refactoring if the lock mechanism changes
#[inline(always)]
pub fn lock(_th: ThLock) {}

/// Checks if this thread can call a Neovim function
///
/// # Panics
///
/// If the current thread is not allowed to call Neovim functions panics with a message that
/// provides the current threads ID and name.
///
/// Can also panic if a dependency with mismatching nvimium versions is used.
pub fn call_check() {
    let allowed = HAS_ACCESS.get();
    if !allowed {
        #[cold]
        #[inline(never)]
        fn yeet() -> ! {
            let th = std::thread::current();
            panic!(
                "thread without access has called neovim function ThreadName={:?} - ThreadId={:?}",
                th.name(),
                th.id(),
            );
        }
        yeet();
    }

    if MAIN_LUA.load(Ordering::Relaxed).is_null() {
        #[cold]
        #[inline(never)]
        fn version_mismatch() -> ! {
            panic!(
                "Lua pointer is not initialized, this is likely triggered due to \
                version mismatches with dependencies that use nvimium.\n
                supported nvimium version: {}",
                env!("CARGO_PKG_VERSION")
            )
        }
        version_mismatch();
    }
}

/// Same as [`call_check`] except it returns a bool value indicating if it is safe to call a neovim
/// function
#[inline(always)]
pub fn can_call() -> bool {
    HAS_ACCESS.get() && !MAIN_LUA.load(Ordering::Relaxed).is_null()
}

impl Drop for ThLock {
    fn drop(&mut self) {
        // the drop call is actually just a best effort to disable the lock, in user code
        // [`scoped`] should be used as it will catch any panic and lock access
        let _ = HAS_ACCESS.try_with(|c| c.set(false));
    }
}

/// Calls the provided function wrapped with [`catch_unwind`] and resumes unwinding after revoking
/// the threads access
///
/// # Safety
///
/// Same safety requirements of [`unlock`] apply here.
pub unsafe fn scoped<F: Fn(A) -> R, A, R>(f: F, arg: A) -> R {
    let th_lock = unsafe { unlock() };
    let ret = catch_unwind(AssertUnwindSafe(|| f(arg)));
    lock(th_lock);
    match ret {
        Ok(r) => r,
        Err(err) => {
            resume_unwind(err);
        }
    }
}

/// Calls the provided function same way as [`scoped`] but restores access if it already has
/// access.
///
/// Only relevant when a callback indirectly calls another callback such as a user command.
///
/// # Safety
///
/// Same as [`scoped`] but does not correct an incorrect state if it access was not revoked on
/// return.
pub unsafe fn scoped_callback<F: Fn(A) -> R, A, R>(f: F, arg: A) -> R {
    let can_call = can_call();
    let th_lock = ManuallyDrop::new(unsafe { unlock() });
    let ret = catch_unwind(AssertUnwindSafe(|| f(arg)));
    if !can_call {
        ManuallyDrop::into_inner(th_lock);
    }
    match ret {
        Ok(r) => r,
        Err(err) => {
            resume_unwind(err);
        }
    }
}

// this also serves as a check to ensure that this version of nvimium was initialized
//
// a library that is a dependent of nvimium will likely want to call neovims C function
// since we dont have a way to ensure that a library dependent on nvimium is using a matching
// version, this static should be checked if it contains a null pointer
static MAIN_LUA: AtomicPtr<lua_State> = AtomicPtr::new(core::ptr::null_mut());
thread_local! {static LUA_PTR: Cell<Option<NonNull<lua_State>>> = const { Cell::new(None) }}

/// Initialize the lua pointer for the main thread
///
/// # Safety
///
/// The pointer must point to the main Lua instance.
/// This is almost always the Lua pointer provided when loading a plugin.
#[inline(always)]
pub unsafe fn init_main_lua_ptr(ptr: *mut lua_State) {
    MAIN_LUA.store(ptr, Ordering::Relaxed);
}

/// Initialize the lua pointer for the current thread
///
/// If the main lua pointer has not been initialized yet, this will initialize the main pointer as
/// well.
///
/// # Safety
///
/// The exact safety requirements depend on the call site, but the pointer must always point to a Lua
/// instance.
pub unsafe fn init_lua_ptr(ptr: *mut lua_State) {
    let _ = MAIN_LUA.compare_exchange(
        core::ptr::null_mut(),
        ptr,
        Ordering::Acquire,
        Ordering::Relaxed,
    );
    LUA_PTR.set(NonNull::new(ptr));
}

pub struct LuaPtr {
    ptr: *mut lua_State,
    should_unset: bool,
}

impl LuaPtr {
    pub fn as_ptr(&mut self) -> *mut lua_State {
        self.ptr
    }
}

impl Drop for LuaPtr {
    fn drop(&mut self) {
        if self.should_unset {
            LUA_PTR.take();
        }
    }
}
pub fn get_lua_ptr() -> LuaPtr {
    let (ptr, should_unset) = if let Some(ptr) = LUA_PTR.get() {
        (ptr.as_ptr(), true)
    } else {
        (MAIN_LUA.load(Ordering::Relaxed), false)
    };
    LuaPtr { ptr, should_unset }
}

#[cfg(test)]
mod tests {
    use std::{panic::catch_unwind, ptr::NonNull};

    use crate::{HAS_ACCESS, call_check, init_main_lua_ptr, scoped};

    #[test]
    fn scoped_gives_access() {
        // in this context the pointer is not used
        // we need a non null pointer to pass call_check
        unsafe { init_main_lua_ptr(NonNull::dangling().as_ptr()) };
        unsafe {
            scoped(
                |_: ()| {
                    call_check();
                },
                (),
            )
        };
    }

    #[test]
    fn panic_revokes_access() {
        let res = catch_unwind(|| unsafe {
            scoped(
                |_| {
                    panic!("some panic");
                },
                (),
            )
        });
        assert!(res.is_err());
        assert!(!HAS_ACCESS.get());
    }
}
