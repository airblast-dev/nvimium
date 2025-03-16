use macros::masked_builder;

masked_builder!(
    #[repr(C)]
    #[derive(Clone, Debug)]
    pub struct GetMarkOpts {}
);
