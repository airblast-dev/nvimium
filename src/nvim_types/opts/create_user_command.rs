use std::error::Error;

use crate::{
    macros::{
        masked_builder::masked_builder,
        nv_enum::{nv_obj_ref_enum, nv_str_enum},
        zeroed_default::zeroed_default,
    },
    nvim_types::{
        AsThinString, Boolean, Integer, Object, ThinString,
        args::user_command_complete_cb::UserCommandCompleteArgs,
        lua::Function,
        object::{ObjectRef, ObjectTag},
        object_subs::BoolOrInteger,
    },
    th,
};

nv_obj_ref_enum!(
    #[derive(Clone, Copy, Debug)]
    pub enum UserCommandAddr {
        Arguments = ObjectRef::new_th(th!("arguments")),
        Lines = ObjectRef::new_th(th!("lines")),
        LoadedBuffers = ObjectRef::new_th(th!("loaded_buffers")),
        Tabs = ObjectRef::new_th(th!("tabs")),
        Buffers = ObjectRef::new_th(th!("buffers")),
        Windows = ObjectRef::new_th(th!("windows")),
        QuickFix = ObjectRef::new_th(th!("quickfix")),
        Other = ObjectRef::new_th(th!("other")),
    }
);

nv_str_enum!(
    /// The kind of completion for arguments
    ///
    /// The structs associated constants should be used when passing this to neovim functions or
    /// options.
    ///
    /// See Neovim's [https://neovim.io/doc/user/map.html#%3Acommand-complete](command-complete) docs
    /// on what the values provide what kind of completion.
    #[derive(Clone, Copy, Debug)]
    pub enum UserCommandCompleteKind {
        ArgList = "arglist",
        AuGroup = "augroup",
        Buffer = "buffer",
        CheckHealth = "checkhealth",
        Colors = "colors",
        Command = "command",
        Compiler = "compiler",
        Custom = "custom",
        CustomList = "customlist",
        LuaFunction = "<Lua function>",
        DiffBuffer = "diff_buffer",
        Directories = "dir",
        Environment = "environment",
        Event = "event",
        Expression = "expression",
        File = "file",
        FileInPath = "file_in_path",
        FileType = "filetype",
        FileTypeCmd = "filetypecmd",
        Function = "function",
        Help = "help",
        Highlight = "highlight",
        History = "history",
        Keymap = "keymap",
        Locale = "locale",
        Lua = "lua",
        MapClear = "mapclear",
        Mappings = "mapping",
        Menus = "menu",
        Messages = "messages",
        Syntax = "syntax",
        SynTime = "syntime",
        Option = "option",
        PackAdd = "packadd",
        Runtime = "runtime",
        ShellCmd = "shellcmd",
        ShellCmdLine = "shellcmdline",
        Sign = "sign",
        Tag = "tag",
        TagListFiles = "tag_listfiles",
        User = "user",
        UserVars = "var",
        BreakPoint = "breakpoint",
        ScriptNames = "scriptnames",
        DirsInCdPath = "dir_in_path",
    }
);

#[repr(C)]
pub struct UserCommandComplete(ObjectRef<'static>);

impl<
    R: 'static + AsThinString,
    E: 'static + Error,
    F: 'static + for<'a> Fn(UserCommandCompleteArgs<'a>) -> Result<R, E> + Unpin,
> From<F> for UserCommandComplete
{
    fn from(value: F) -> Self {
        let lref = Function::wrap(value).into_luaref();
        Self(ObjectRef::from(lref))
    }
}

impl From<UserCommandCompleteKind> for UserCommandComplete {
    fn from(value: UserCommandCompleteKind) -> Self {
        Self(ObjectRef::new_th(value.as_enum_str()))
    }
}

impl Drop for UserCommandComplete {
    fn drop(&mut self) {
        if self.0.tag == ObjectTag::LuaRef {
            unsafe { core::mem::ManuallyDrop::drop(&mut self.0.val.lua_ref) }
        }
    }
}

nv_obj_ref_enum!(
    #[derive(Clone, Copy, Debug)]
    pub enum UserCommandNarg {
        Zero = ObjectRef::new_int(0),
        One = ObjectRef::new_int(1),
        ZeroOrMore = ObjectRef::new_th(th!("*")),
        ZeroOrOne = ObjectRef::new_th(th!("?")),
        OneOrMore = ObjectRef::new_th(th!("+")),
    }
);

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
        let r = match value {
            UserCommandRange::Allowed => ObjectRef::new_bool(true),
            UserCommandRange::WholeBuffer => ObjectRef::new_th(th!("%")),
            UserCommandRange::AllowedDefault(n) => ObjectRef::new_int(n),
        };

        Self(r)
    }
}

masked_builder!(
    #[repr(C)]
    pub struct CreateUserCommandOpts<'a> {
        #[builder(nv_obj_ref_enum)]
        addr: UserCommandAddr,
        bang: Boolean,
        bar: Boolean,
        #[builder(into)]
        complete: UserCommandComplete,
        count: BoolOrInteger,
        // with a neovim string
        #[builder(skip)]
        desc: ObjectRef<'a>,
        force: Boolean,
        keepscript: Boolean,
        #[builder(nv_obj_ref_enum)]
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
);

zeroed_default!(CreateUserCommandOpts<'_>);

impl<'a> CreateUserCommandOpts<'a> {
    pub fn desc<TH: Into<ThinString<'a>>>(&mut self, desc: TH) -> &mut Self {
        const MASK: u64 = 1 << builder::MASK_OFFSETS[5];
        if self.mask & MASK == MASK {
            unsafe {
                self.desc.assume_init_drop();
            }
        }
        self.desc.write(ObjectRef::from(desc.into()));
        self.mask |= MASK;
        self
    }
    pub fn range<I: Into<UserCommandRange>>(&mut self, range: I) -> &mut Self {
        let range: UserCommandRange = range.into();
        const MASK: u64 = 1 << builder::MASK_OFFSETS[10];
        if self.mask & MASK == MASK {
            unsafe {
                self.desc.assume_init_drop();
            }
        }
        self.range.write(range.into());
        self.mask |= MASK;
        self
    }
}
