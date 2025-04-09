use core::{marker::PhantomData, mem::ManuallyDrop};

#[repr(transparent)]
pub struct Borrowed<'a, T>(ManuallyDrop<T>, PhantomData<&'a T>);

impl<'a, T> Borrowed<'a, T> {
    #[doc(hidden)]
    pub const fn new(value: &'a T) -> Self {
        let val = unsafe { (value as *const T).cast::<ManuallyDrop<T>>().read() };
        Self(val, PhantomData)
    }

    #[inline(always)]
    pub const fn as_ref(&self) -> &T {
        let ptr: *const ManuallyDrop<T> = &raw const self.0;
        unsafe { (ptr as *const T).as_ref().unwrap_unchecked() }
    }
}

impl<T> AsRef<T> for Borrowed<'_, T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
