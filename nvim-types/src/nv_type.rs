pub(super) trait NvType: Sealed {}
impl <T> NvType for T where T: Sealed {}

trait Sealed {}
impl<T: NvType> Sealed for crate::kvec::KVec<T> {}
