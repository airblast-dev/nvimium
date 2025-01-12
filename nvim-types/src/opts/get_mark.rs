use macros::masked_builder;

masked_builder!(
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default)]
    pub struct GetMarkOpts {}
);
