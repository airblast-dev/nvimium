use std::mem::ManuallyDrop;

use crate::{array::Array, dictionary::Dictionary};

pub struct Object {
    kind: u32,
    inner: Inner,
}

impl Clone for Object {
    fn clone(&self) -> Self {
        todo!("impl clone for object")
    }
}

union Inner {
    array: ManuallyDrop<Array>,
    dict: ManuallyDrop<Dictionary>,
}
