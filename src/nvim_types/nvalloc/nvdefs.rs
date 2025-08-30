use libc::c_char;

unsafe extern "C" {
    #[link_name = "e_outofmem"]
    pub static E_OUTOFMEM: *const c_char;
    /// Attempts to recover from an allocation error by freeing extra memory that neovim is holding.
    ///
    /// Intended to be used with this libraries xmalloc definition and its exported allocator
    #[cold]
    pub fn try_to_free_memory();
    #[cold]
    pub fn preserve_exit(msg: *const c_char) -> !;
}
