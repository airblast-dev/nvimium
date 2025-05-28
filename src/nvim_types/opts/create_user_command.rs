use std::error::Error;

use crate::{
    masked_builder,
    nvim_types::{
        AsThinString, Boolean, Integer, Object, ThinString,
        args::user_command_complete::UserCommandCompleteArgs,
        lua::Function,
        object::{ObjectRef, ObjectTag},
        object_subs::BoolOrInteger,
    },
    th,
};

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct UserCommandAddr(ObjectRef<'static>);

macro_rules! uca {
    ($vis:vis $name:ident = $val:literal) => {
        $vis const $name: UserCommandAddr = UserCommandAddr(unsafe { ObjectRef::new(ObjectTag::String, &th!($val) ) });
    };
}
impl UserCommandAddr {
    uca!(pub ARGUMENTS = "arguments");
    uca!(pub LINES = "lines");
    uca!(pub LOADED_BUFFERS = "loaded_buffers");
    uca!(pub TABS = "tabs");
    uca!(pub BUFFERS = "buffers");
    uca!(pub WINDOWS = "windows");
    uca!(pub QUICKFIX = "quickfix");
    uca!(pub OTHER = "other");
}

/// The kind of completion for arguments
///
/// The structs associated constants should be used when passing this to neovim functions or
/// options.
///
/// See Neovim's [https://neovim.io/doc/user/map.html#%3Acommand-complete](command-complete) docs
/// on what the values provide what kind of completion.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct UserCommandCompleteKind(
    ThinString<'static>,
    // padding to get it to the size of an Object
    usize,
);

macro_rules! ucck {
    ($(#[$attr:meta])* $vis:vis $name:ident = $val:literal) => {
        $(#[$attr])*
        $vis const $name: UserCommandCompleteKind = UserCommandCompleteKind(th!($val), 0);
    };
}

impl UserCommandCompleteKind {
    ucck!(pub ARG_LIST = "arglist");
    ucck!(pub AUGROUP = "augroup");
    ucck!(pub BUFFER = "buffer");
    ucck!(pub CHECKHEALTH = "checkhealth");
    ucck!(pub COLORS = "colors");
    ucck!(pub COMMAND = "command");
    ucck!(pub COMPILER = "compiler");
    ucck!(pub CUSTOM = "custom");
    ucck!(pub CUSTOM_LIST = "customlist");
    ucck!(pub LUA_FUNCTION = "<Lua function>");
    ucck!(pub DIFF_BUFFER = "diff_buffer");
    ucck!(pub DIRECTORIES = "dir");
    ucck!(pub ENV_VARS = "environment");
    ucck!(pub EVENT = "event");
    ucck!(pub EXPRESSION = "expression");
    ucck!(pub FILE = "file");
    ucck!(pub FILE_IN_PATH = "file_in_path");
    ucck!(pub FILETYPE = "filetype");
    ucck!(pub FILETYPE_CMD = "filetypecmd");
    ucck!(pub FUNCTION = "function");
    ucck!(pub HELP = "help");
    ucck!(pub HIGHLIGHT = "highlight");
    ucck!(pub HISTORY = "history");
    ucck!(pub KEYMAP = "keymap");
    ucck!(
        /// Only available on platforms that support libint
        ///
        /// See: https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/libintl.h.html
        pub LOCALE = "locale");
    ucck!(pub LUA = "lua");
    ucck!(pub MAP_CLEAR = "mapclear");
    ucck!(pub MAPPINGS = "mapping");
    ucck!(pub MENUS = "menu");
    ucck!(pub MESSAGES = "messages");
    ucck!(pub SYNTAX = "syntax");
    ucck!(pub SYNTIME = "syntime");
    ucck!(pub OPTION = "option");
    ucck!(pub PACKADD = "packadd");
    ucck!(pub RUNTIME = "runtime");
    ucck!(pub SHELL_CMD = "shellcmd");
    ucck!(pub SHELL_CMD_LINE = "shellcmdline");
    ucck!(pub SIGN = "sign");
    ucck!(pub TAG = "tag");
    ucck!(pub TAGS_LISTFILES = "tag_listfiles");
    ucck!(pub USER = "user");
    ucck!(pub USER_VARS = "var");
    ucck!(pub BREAKPOINT = "breakpoint");
    ucck!(pub SRCRIPT_NAMES = "scriptnames");
    ucck!(pub DIRS_IN_CDPATH = "dir_in_path");
}

#[derive(Debug)]
#[repr(transparent)]
pub struct UserCommandComplete(ObjectRef<'static>);

impl From<UserCommandCompleteKind> for UserCommandComplete {
    fn from(value: UserCommandCompleteKind) -> Self {
        Self(unsafe { ObjectRef::new(ObjectTag::String, &value.0) })
    }
}

impl<
    R: 'static + AsThinString,
    E: 'static + Error,
    F: 'static + for<'a> Fn(UserCommandCompleteArgs<'a>) -> Result<R, E>,
> From<F> for UserCommandComplete
{
    fn from(value: F) -> Self {
        let lref = Function::wrap(value).into_luaref();
        Self(unsafe { ObjectRef::new_moved(ObjectTag::LuaRef, lref) })
    }
}

const _: () = assert!(
    size_of::<UserCommandComplete>() == size_of::<Object>()
        && align_of::<UserCommandComplete>() == align_of::<Object>()
);

/// The number of arguments accepted by the user command
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct UserCommandNarg(ObjectRef<'static>);

impl UserCommandNarg {
    pub const ZERO: UserCommandNarg =
        UserCommandNarg(unsafe { ObjectRef::new(ObjectTag::Integer, &0) });
    pub const ONE: UserCommandNarg =
        UserCommandNarg(unsafe { ObjectRef::new(ObjectTag::Integer, &1) });
    pub const ZERO_OR_MORE: UserCommandNarg =
        UserCommandNarg(unsafe { ObjectRef::new(ObjectTag::String, &th!("*")) });
    pub const ZERO_OR_ONE: UserCommandNarg =
        UserCommandNarg(unsafe { ObjectRef::new(ObjectTag::String, &th!("?")) });
    pub const ONE_OR_MORE: UserCommandNarg =
        UserCommandNarg(unsafe { ObjectRef::new(ObjectTag::String, &th!("+")) });
}

#[derive(Clone, Debug)]
pub enum UserCommandRange {
    /// -range
    Allowed,
    /// -range=%
    WholeBuffer,
    /// -range=N
    AllowedDefault(Integer),
}

#[derive(Clone, Debug)]
#[repr(transparent)]
struct UserCommandRangeInner(ObjectRef<'static>);
impl From<UserCommandRange> for UserCommandRangeInner {
    fn from(value: UserCommandRange) -> Self {
        let r = unsafe {
            match value {
                UserCommandRange::Allowed => ObjectRef::new(ObjectTag::Bool, &true),
                UserCommandRange::WholeBuffer => ObjectRef::new(ObjectTag::String, &th!("%")),
                UserCommandRange::AllowedDefault(n) => ObjectRef::new(ObjectTag::Integer, &n),
            }
        };

        Self(r)
    }
}

masked_builder! {
    #[repr(C)]
    pub struct CreateUserCommandOpts<'a> {
        addr: UserCommandAddr,
        bang: Boolean,
        bar: Boolean,
        complete: UserCommandComplete,
        count: BoolOrInteger,
        // with a neovim string
        #[builder(skip)]
        desc: ObjectRef<'a>,
        force: Boolean,
        keepscript: Boolean,
        nargs: UserCommandNarg,
        // with a lua ref
        preview: Object,
        #[builder(skip)]
        // unlike the others this is skipped because the generated function signature contains
        // the internal struct as a trait bound
        // this type is kind of annoying as its hard to express the requirements of this value with the type
        // system
        range: UserCommandRangeInner,
        register: Boolean,
    }
}

impl<'a> CreateUserCommandOpts<'a> {
    pub fn desc<TH: 'a + AsThinString>(&mut self, desc: TH) -> &mut Self {
        if self.mask & (1 << 7) == 1 << 7 {
            unsafe {
                self.desc.assume_init_drop();
            }
        }
        self.desc
            .write(unsafe { ObjectRef::new(ObjectTag::String, &desc.as_thinstr()) });
        self.mask |= 1 << 7;
        self
    }
    pub fn range<I: Into<UserCommandRange>>(&mut self, range: I) -> &mut Self {
        let range: UserCommandRange = range.into();
        if self.mask & (1 << 12) == 1 << 12 {
            unsafe {
                self.desc.assume_init_drop();
            }
        }
        self.range.write(range.into());
        self.mask |= 1 << 12;
        self
    }
}
