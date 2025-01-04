use std::mem::ManuallyDrop;

use crate::{array::Array, dictionary::Dictionary};

pub struct Object {
    kind: u32,
    inner: Inner,
}

union Inner {
    array: ManuallyDrop<Array>,
    dict: ManuallyDrop<Dictionary>,
}
