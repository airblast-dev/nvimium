use crate::{
    macros::{
        masked_builder::masked_builder,
        nv_enum::{nv_obj_ref_enum, nv_str_enum},
 zeroed_default::zeroed_default,
    },
    nvim_types::{object::ObjectRef, Array, Boolean, Float, Integer, Object, Window},
    th,
};

nv_str_enum!(
    #[derive(Clone, Copy, Debug)]
    pub enum Anchor {
        NorthWest = "NW",
        NorthEast = "NE",
        SouthWest = "SW",
        SouthEast = "SE",
    }
);

nv_str_enum!(
    #[derive(Clone, Copy, Debug)]
    pub enum Relative {
        Cursor = "cursor",
        Editor = "editor",
        LastStatus = "laststatus",
        Mouse = "mouse",
        Tabline = "tabline",
        Win = "win",
    }
);

nv_str_enum!(
    #[derive(Clone, Copy, Debug)]
    pub enum Split {
        Left = "left",
        Right = "right",
        Above = "above",
        Below = "below",
    }
);

nv_obj_ref_enum!(
    #[derive(Clone, Copy, Debug)]
    pub enum Border {
        None = ObjectRef::new_th(th!("none")),
        Single = ObjectRef::new_th(th!("single")),
        Double = ObjectRef::new_th(th!("double")),
        Rounded = ObjectRef::new_th(th!("rounded")),
        Solid = ObjectRef::new_th(th!("solid")),
        Shadow = ObjectRef::new_th(th!("shadow")),
    }
);

nv_str_enum!(
    #[derive(Clone, Copy, Debug)]
    pub enum TitlePos {
        Center = "center",
        Left = "left",
        Right = "right",
    }
);

nv_str_enum!(
    #[derive(Clone, Copy, Debug)]
    pub enum FooterPos {
        Center = "center",
        Left = "left",
        Right = "right",
    }
);

nv_str_enum!(
    #[derive(Clone, Copy, Debug)]
    pub enum Style {
        Minimal = "minimal",
    }
);

masked_builder!(
    #[repr(C)]
    pub struct WinConfig {
        row: Float,
        col: Float,
        width: Integer,
        height: Integer,
        #[builder(nv_str_enum)]
        anchor: Anchor,
        #[builder(nv_str_enum)]
        relative: Relative,
        #[builder(nv_str_enum)]
        split: Split,
        win: Window,
        // TODO: use better type
        bufpos: Array,
        external: Boolean,
        focusable: Boolean,
        mouse: Boolean,
        vertical: Boolean,
        zindex: Integer,
        #[builder(nv_obj_ref_enum)]
        border: Border,
        // TODO: use better type and support highlighted titles
        #[builder(skip)]
        title: Object,
        #[builder(nv_str_enum)]
        title_pos: TitlePos,
        // TODO: use better type and support highlighted footers
        #[builder(skip)]
        footer: Object,
        #[builder(nv_str_enum)]
        footer_pos: FooterPos,
        #[builder(nv_str_enum)]
        style: Style,
        noautocmd: Boolean,
        fixed: Boolean,
        hide: Boolean,
        _cmdline_offset: Integer,
    }
);

zeroed_default!(WinConfig);
