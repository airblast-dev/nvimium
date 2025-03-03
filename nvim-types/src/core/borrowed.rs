use std::{marker::PhantomData, mem::ManuallyDrop};

#[repr(transparent)]
pub struct Borrowed<'a, T>(ManuallyDrop<T>, PhantomData<&'a T>);

impl<'a, T> Borrowed<'a, T> {
    pub(crate) fn new(value: &'a T) -> Self {
        let val = unsafe { (value as *const T).cast::<ManuallyDrop<T>>().read() };
        Self(val, PhantomData)
    }
}

impl<T> AsRef<T> for Borrowed<'_, T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}
