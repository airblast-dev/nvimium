use std::ops::DerefMut;

use nvim_types::{array::Array, borrowed::Borrowed};

fn main() {
    fn from_array() {
        let array = Array::default();
        let borrowed: Borrowed<'_, Array> = array.into();
    }

    fn from_array_mut() {
        let array = Array::default();
        let borrowed: Borrowed<'_, Array> = (&array).into();
        let mut_array = array.deref_mut();
        borrowed.as_ref();
    }
}
