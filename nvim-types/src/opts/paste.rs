#[derive(Clone, Copy, Debug)]
#[repr(i64)]
pub enum PastePhase {
    Start = 1,
    Continue = 2,
    End = 3,
}
