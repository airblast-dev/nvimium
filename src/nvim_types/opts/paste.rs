#[derive(Clone, Debug)]
#[repr(i64)]
pub enum PastePhase {
    Single = -1,
    Start = 1,
    Continue = 2,
    End = 3,
}
