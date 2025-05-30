use core::fmt::Display;

use crate::nvim_types::{AsThinString, NvString, ThinString};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Hash)]
pub enum FeedKeysModeKind {
    #[default]
    Remap = b'm',
    NoRemap = b'n',
    Typed = b't',
    LowLever = b'L',
    Insert = b'i',
    Execute = b'x',
    NoEnd = b'!',
}

impl Display for FeedKeysModeKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Clone, Debug, Default)]
pub struct FeedKeysMode(NvString);

impl FeedKeysMode {
    /// Initialize a new [`FeedKeysMode`]
    ///
    /// The mode of the returned value is [`FeedKeysModeKind::Remap`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a mode kind to the current list of modes
    pub fn push(&mut self, mode: FeedKeysModeKind) {
        self.0.push([mode as u8]);
    }

    /// Get a slice of the current modes as bytes
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_slice()
    }
}

unsafe impl AsThinString for FeedKeysMode {
    fn as_thinstr(&self) -> ThinString<'_> {
        self.0.as_thinstr()
    }
}

impl<T: AsRef<[FeedKeysModeKind]>> From<T> for FeedKeysMode {
    fn from(value: T) -> Self {
        // the cast may change the length of the slice which isn't what we want
        // assert the size to avoid that case
        const _: () = assert!(core::mem::size_of::<FeedKeysModeKind>() == 1);
        Self(unsafe {
            NvString::from(core::mem::transmute::<&[FeedKeysModeKind], &[u8]>(
                value.as_ref(),
            ))
        })
    }
}

#[cfg(all(test, miri))]
mod tests {
    use crate::nvim_types::NvString;

    use super::{FeedKeysMode, FeedKeysModeKind};

    #[test]
    fn feed_keys_mode_from_kind() {
        let kinds = [
            FeedKeysModeKind::Remap,
            FeedKeysModeKind::Execute,
            FeedKeysModeKind::NoEnd,
        ];
        let mode = FeedKeysMode::from(kinds);

        assert_eq!(mode.0, NvString::from("mx!"));
    }
}
