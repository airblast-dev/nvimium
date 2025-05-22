use core::{
    fmt::Debug,
    iter::FusedIterator,
    mem::{self, MaybeUninit},
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
};
use std::alloc::{GlobalAlloc, Layout, handle_alloc_error};

use crate::{GLOBAL_ALLOCATOR, allocator::NvAllocator};
use panics::slice_error;

unsafe impl<T> Sync for KVec<T> where T: Sync {}
unsafe impl<T> Send for KVec<T> where T: Send {}

#[repr(C)]
pub struct KVec<T> {
    pub(super) len: usize,
    pub(super) capacity: usize,
    pub(super) ptr: *mut T,
}

impl<T> Default for KVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> Debug for KVec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.as_slice().iter()).finish()
    }
}

impl<T> KVec<T> {
    pub(super) const T_SIZE: usize = mem::size_of::<T>();
    pub(super) const ZST: bool = Self::T_SIZE == 0;

    /// Initialize an empty [`KVec`]
    ///
    /// Will not allocate until reserve methods are called, or elements are appended.
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            len: 0,
            capacity: 0,
            ptr: core::ptr::null_mut(),
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        if self.is_empty() {
            return &mut [];
        }
        self.slice_check();

        // SAFETY: self has at least one element stored this means self.ptr is non null
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        if self.is_empty() {
            return &mut [];
        }
        self.slice_check();

        // SAFETY: self has at least one element stored this means self.ptr is non null
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }

    /// Returns the uninitialized but allocated space for `T`
    #[inline]
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        // we can't check null pointers consistently in const context
        // since a null pointers capacity must always 0 this is the equivelant
        if self.capacity() == 0 {
            return &mut [];
        }
        self.slice_check();

        // SAFETY: self has at least one element stored this means self.ptr is non null
        unsafe {
            core::slice::from_raw_parts_mut(
                self.ptr.add(self.len).cast::<MaybeUninit<T>>(),
                self.capacity - self.len(),
            )
        }
    }

    /// Returns a pointer to the internal buffer
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr
    }

    #[track_caller]
    #[inline(always)]
    const fn slice_check(&self) {
        // see docs for `std::slice::from_raw_parts*`
        let Some(byte_size) = Self::T_SIZE.checked_mul(self.len()) else {
            slice_error();
        };
        let max_slice_len = isize::MAX as usize;
        if byte_size > max_slice_len {
            slice_error();
        }
    }

    /// Returns the number elements present in the [`KVec`]
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the buffer is empty
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Set the length of the [`KVec`]
    ///
    /// This should usually called after initializing elements via [`KVec::spare_capacity_mut`].
    ///
    /// # Safety
    ///
    /// Calling this function when len items are not initialized may cause undefined behavior.
    #[inline(always)]
    pub unsafe fn set_len(&mut self, len: usize) {
        self.len = len;
    }

    /// Returns the total capacity of the [`KVec`]
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Initialize [`KVec`] with for at least capacity elements.
    ///
    /// Does not allocate if capacity is 0 or T is a ZST.
    ///
    /// # Guarantee's
    ///
    /// This function is guaranteed to at least allocate for `capacity` elements.
    pub fn with_capacity(capacity: usize) -> Self {
        // We shouldn't be storing ZST's in any use case, but if we do, just return a dangling pointer.
        let layout = Layout::array::<T>(capacity).unwrap();
        let ptr = unsafe { NvAllocator::new(true).alloc(Layout::array::<T>(capacity).unwrap()) }
            as *mut T;
        if ptr.is_null() {
            handle_alloc_error(layout);
        }

        Self {
            len: 0,
            capacity,
            ptr,
        }
    }

    /// Reserve space for at least additional elements
    ///
    /// If enough space already exists for the additional elements, this will not allocate.
    /// If you already know the *exact* total number of elements that [`KVec`] will contain, prefer
    /// [`KVec::with_capacity`] or [`KVec::reserve_exact`] as this function will often allocate
    /// extra space.
    ///
    /// # Guarantee's
    ///
    /// Always allocates space for at least len + additional elements.
    pub fn reserve(&mut self, additional: usize) {
        let Some(min_capacity) = self.next_minimum_capacity(additional) else {
            return;
        };
        let new_capacity = min_capacity
            .checked_next_power_of_two()
            .unwrap_or(min_capacity);
        if Self::ZST {
            self.capacity += new_capacity.get();
            return;
        }
        self.realloc(new_capacity.get());
    }

    /// Reserve space for at least additional elements
    ///
    /// If enough space already exists for the additional elements, this will not allocate.
    /// Generally [`KVec::reserve`] should be preferred.
    ///
    /// # Guarantee's
    ///
    /// Always allocates space for at least len + additional elements.
    pub fn reserve_exact(&mut self, additional: usize) {
        if Self::ZST {
            self.capacity += additional;
            return;
        }
        let Some(new_capacity) = self.next_minimum_capacity(additional) else {
            return;
        };

        self.realloc(new_capacity.get());
    }

    /// The remaining capacity in [`KVec`]
    #[inline(always)]
    fn remaining_capacity(&self) -> usize {
        assert!(self.capacity() >= self.len());
        self.capacity() - self.len()
    }

    /// Returns the minimum new capacity.
    ///
    /// If additional elements already fits, this returns None.
    #[inline]
    fn next_minimum_capacity(&self, additional: usize) -> Option<NonZeroUsize> {
        let remaining = self.remaining_capacity();

        // check if we already have enough space
        if remaining >= additional {
            return None;
        }
        let new_capacity = self.capacity + additional - remaining;
        assert_ne!(new_capacity, 0);
        // SAFETY: additional is always bigger than remaining which is checked above
        unsafe { Some(NonZeroUsize::new_unchecked(new_capacity)) }
    }

    // Inlined as most of the conditions are likely to be checked in call sites.
    #[inline(always)]
    fn realloc(&mut self, new_capacity: usize) {
        if self.capacity() != new_capacity {
            self.ptr = unsafe {
                GLOBAL_ALLOCATOR.realloc(
                    self.ptr as _,
                    // SAFETY: already validated during allocation
                    Layout::array::<T>(self.capacity()).unwrap_unchecked(),
                    Layout::array::<T>(new_capacity).unwrap().size(),
                ) as *mut T
            };
        }
        self.capacity = new_capacity;
    }

    /// Push an element to the end of the [`KVec`]
    ///
    /// When pushing multiple elements in the [`KVec`] prefer it's [`Extend`] implementation or
    /// [`KVec::extend_from_slice`] if cloning from a slice to avoid extra allocation visits.
    pub fn push(&mut self, element: T) {
        self.reserve_exact(1);

        // SAFETY: reserve_exact guarantees that at least space for one element will be allocated
        unsafe { self.push_unchecked(element) };
    }

    /// Push T without checking for capacity.
    ///
    /// # Safety
    ///     
    /// Callers must guarantee that enough space is allocated. It is undefined behavior otherwise.
    #[doc(hidden)]
    pub unsafe fn push_unchecked(&mut self, element: T) {
        debug_assert!(
            self.len() < self.capacity(),
            "called KVec::push_unchecked without enough capacity"
        );
        if !Self::ZST {
            unsafe {
                self.ptr.add(self.len()).write(element);
            }
        }
        unsafe { self.set_len(self.len() + 1) };
    }

    /// Append the elements to the end by cloning
    ///
    /// To use with iterators see [`KVec`]'s [`Extend::extend`] implementation.
    pub fn extend_from_slice(&mut self, s: &[T])
    where
        T: Clone,
    {
        self.reserve_exact(s.len());
        if !Self::ZST {
            // SAFETY: we have reserved the required space, we now have enough provenance to
            // get_unchecked
            let spare = unsafe { self.spare_capacity_mut().get_unchecked_mut(..s.len()) };
            for i in 0..s.len() {
                spare[i].write(s[i].clone());
            }
        }

        // SAFETY: s.len() items have been initialized
        unsafe { self.set_len(self.len() + s.len()) };
    }

    /// Removes and returns the element at index
    ///
    /// This is done by shifting all elements after it to the left.
    /// Because this shifts over the elements after `index`, this is an *O*(*n*) operation.
    /// If you don't need the elements order to be preserved, use [`KVec::swap_remove`] instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn remove(&mut self, index: usize) -> T {
        let len = self.len();

        #[cold]
        #[track_caller]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("removal index (is {index}) should be < len (is {len})");
        }

        if index >= self.len() {
            assert_failed(index, len)
        }

        unsafe {
            let ptr = self.ptr.add(index);
            let rem = ptr.read();
            #[cfg(miri)]
            core::ptr::copy(ptr.add(1), ptr, len - index - 1);
            // memmove is not supported by miri so we have to feature gate it
            #[cfg(not(miri))]
            libc::memmove(
                ptr.cast(),
                ptr.add(1).cast(),
                (len - index - 1) * Self::T_SIZE,
            );

            self.set_len(len - 1);
            rem
        }
    }

    /// Removes and returns the element at index
    ///
    /// The element at `index` is replaced by the last element. This does not preserve the order of
    /// elements, but is *O*(*1*). If you want to preserve the order of the elements use
    /// [`KVec::remove`] instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn swap_remove(&mut self, index: usize) -> T {
        let len = self.len();

        #[cold]
        #[track_caller]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("removal index (is {index}) should be < len (is {len})");
        }

        if index >= self.len() {
            assert_failed(index, len)
        }

        unsafe {
            let ptr = self.ptr.add(index);
            let rem = ptr.read();

            // if false no need to swap values as it is the last value, since we store the value in
            // "rem" its drop call will also be performed, making it safe to set our new length
            if len - 1 > index {
                libc::memcpy(ptr.cast(), self.ptr.add(len - 1).cast(), Self::T_SIZE);
            }
            self.set_len(len - 1);
            rem
        }
    }

    /// Truncates the [`KVec`] to be the length of `to`
    ///
    /// This does not modify the capacity or reallocate.
    /// The new length is guaranteed to be `to` after this function is called.
    /// If `to` is greater than [`KVec::len`] calling this function does nothing.
    pub fn truncate(&mut self, to: usize) {
        if to >= self.len() {
            return;
        }

        let len = self.len();
        // SAFETY: we have checked if our length is zero above meaning we cant have a null pointer
        unsafe {
            // set the length to zero in case one of the drops panic
            self.set_len(0);
            // len - to cannot overflow due to the condition above
            core::ptr::slice_from_raw_parts_mut(self.as_ptr().add(to), len - to).drop_in_place();
            self.set_len(to);
        }
    }
}

impl<T: Clone> Clone for KVec<T> {
    fn clone(&self) -> Self {
        let mut new = KVec::with_capacity(self.len());
        new.extend_from_slice(self.as_slice());
        new
    }

    fn clone_from(&mut self, source: &Self) {
        let mid_len = self.len().min(source.len());
        self.reserve_exact(source.len().saturating_sub(self.capacity()));
        self.truncate(mid_len);
        self[0..mid_len].clone_from_slice(&source[0..mid_len]);
        self.extend_from_slice(&source[mid_len..]);
    }
}

impl<T> Deref for KVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for KVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T> Extend<T> for KVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut iter = iter.into_iter();
        while let Some(element) = iter.next() {
            self.reserve(iter.size_hint().0.max(1));
            unsafe { self.push_unchecked(element) };
        }
    }
}

impl<T: Clone> From<&[T]> for KVec<T> {
    fn from(value: &[T]) -> Self {
        let mut kv = Self::new();
        kv.extend_from_slice(value);
        kv
    }
}

impl<T> FromIterator<T> for KVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut kv = Self::new();
        kv.extend(iter);
        kv
    }
}

impl<T: Eq> Eq for KVec<T> {}
impl<Item: PartialEq> PartialEq for KVec<Item> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<Item: PartialEq> PartialEq<[Item]> for KVec<Item> {
    fn eq(&self, other: &[Item]) -> bool {
        self.as_slice() == other
    }
}

impl<Item: PartialEq> PartialEq<KVec<Item>> for [Item] {
    fn eq(&self, other: &KVec<Item>) -> bool {
        other.as_slice() == self
    }
}

pub struct Iter<T> {
    start_ptr: *mut T,
    start_pos: usize,
    end_pos: usize,
    capacity: usize,
}

impl<T> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.start_pos >= self.end_pos {
            return None;
        }

        let item = unsafe { self.start_ptr.add(self.start_pos).read() };
        self.start_pos += 1;

        Some(item)
    }
}

impl<T> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start_pos >= self.end_pos {
            return None;
        }

        self.end_pos -= 1;
        Some(unsafe { self.start_ptr.add(self.end_pos).read() })
    }
}

impl<T> FusedIterator for Iter<T> {}

impl<T> Drop for Iter<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.start_ptr.is_null() {
                core::ptr::slice_from_raw_parts_mut(
                    self.start_ptr.add(self.start_pos),
                    self.end_pos - self.start_pos,
                )
                .drop_in_place();
            }
            crate::GLOBAL_ALLOCATOR.dealloc(
                self.start_ptr as _,
                Layout::array::<T>(self.capacity).unwrap_unchecked(),
            );
        }
    }
}

impl<T> IntoIterator for KVec<T> {
    type Item = T;
    type IntoIter = Iter<T>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = Iter {
            start_pos: 0,
            end_pos: self.len(),
            start_ptr: self.as_ptr(),
            capacity: self.capacity(),
        };
        core::mem::forget(self);

        iter
    }
}

impl<T> Drop for KVec<T> {
    fn drop(&mut self) {
        if Self::ZST {
            return;
        }
        let cap = self.capacity();
        unsafe {
            let len = self.len();
            // set the length to zero in case the drops panic
            self.set_len(0);
            // SAFETY: if capacity is greater than zero we have an allocated pointer which is non
            // null
            if cap > 0 {
                core::ptr::slice_from_raw_parts_mut(self.as_ptr(), len).drop_in_place();
                GLOBAL_ALLOCATOR.dealloc(
                    self.ptr as _,
                    Layout::array::<T>(self.capacity()).unwrap_unchecked(),
                );
            }
        }
    }
}

const _: () = assert!(
    mem::size_of::<KVec<[u32; 1]>>()
        == mem::size_of::<usize>() * 2 + mem::size_of::<*mut [u32; 10]>()
);

#[cfg(test)]
mod tests {
    use core::num::NonZeroUsize;

    use std::string::String;

    type KVec = super::KVec<String>;

    #[test]
    fn new() {
        let kv = KVec::new();
        assert_eq!(kv.len(), 0);
        assert_eq!(kv.capacity(), 0);
    }

    #[test]
    fn clone() {
        let v = KVec::from_iter([1, 2, 3_u8].map(|b| b.to_string()));
        let a = v.clone();
        assert_eq!(v, a);

        let mut v = KVec::default();
        v.clone_from(&a);
        assert_eq!(v, a);

        let a = v.clone();
        assert_eq!(v, a);

        let v = KVec::default();
        let mut a = KVec::default();
        a.clone_from(&v);
        assert_eq!(v, a);

        let v = KVec::from_iter([1, 2, 3, 4, 5].map(|b| b.to_string()));
        let a = KVec::from_iter([6, 7].map(|b| b.to_string()));
        {
            let mut v = v.clone();
            v.clone_from(&a);
            assert_eq!(v.as_slice(), [6, 7].map(|b| b.to_string()).as_slice());
            assert_eq!(v, a);
        }
        {
            let mut a = a.clone();
            a.clone_from(&v);
            assert_eq!(
                a.as_slice(),
                [1, 2, 3, 4, 5].map(|b| b.to_string()).as_slice()
            );
            assert_eq!(a, v);
        }
    }

    #[test]
    fn as_slice() {
        let mut kv = KVec::new();
        assert_eq!(kv.as_slice(), &[] as &[String]);
        kv.push(String::from("Hello, World!"));
        assert_eq!(kv.as_slice(), &[String::from("Hello, World!")] as &[String]);
    }

    #[test]
    fn len() {
        let mut kv = KVec::new();
        assert!(kv.len() + kv.len == 0);
        kv.reserve(2);
        assert!(kv.len() + kv.len == 0);
    }

    #[test]
    fn capacity() {
        let mut kv = KVec::new();
        assert!(kv.capacity + kv.capacity() == 0);
        kv.reserve(2);
        assert_eq!(kv.capacity, 2);
        assert_eq!(kv.capacity(), 2);
    }

    #[test]
    fn with_capacity() {
        let kv = KVec::with_capacity(10);
        assert_eq!(kv.len(), 0);
        assert_eq!(kv.capacity(), 10);
    }

    #[test]
    fn reserve() {
        let mut kv = KVec::new();

        // should not change ptr
        let ptr = kv.as_ptr();
        kv.reserve(0);
        assert_eq!(kv.as_ptr(), ptr);

        // should set the capacity to the next power of two
        // the pointer should also change since at this stage it should be a non null pointer
        let ptr = kv.as_ptr();
        kv.reserve(10);
        assert_ne!(ptr, kv.as_ptr());
        assert_eq!(kv.len(), 0);
        assert_eq!(kv.capacity(), 16);

        // we already allocated, everything should be the same
        kv.reserve(10);
        assert_ne!(ptr, kv.as_ptr());
        assert_eq!(kv.len(), 0);
        assert_eq!(kv.capacity(), 16);

        // check if reserve 0 changes the pointer, essentially checks if we visit the allocator
        // (miri will always change the pointer on realloc regardless of the block size)
        let ptr = kv.as_ptr();
        kv.reserve(0);
        assert_eq!(kv.as_ptr(), ptr);
    }

    #[test]
    fn reserve_exact() {
        // just read the comments in the reserve tests, pretty much the same story
        let mut kv = KVec::new();

        let ptr = kv.as_ptr();
        kv.reserve_exact(0);
        assert_eq!(kv.as_ptr(), ptr);
        assert_eq!(kv.len(), 0);

        kv.reserve_exact(1);
        assert_ne!(kv.as_ptr(), ptr);
        assert_eq!(kv.len(), 0);

        // the ptr may not change if the allocator extends the block, with miri the pointer always
        // changes on allocator visits
        #[cfg(miri)]
        let ptr = kv.as_ptr();
        kv.reserve_exact(2);
        #[cfg(miri)]
        assert_ne!(kv.as_ptr(), ptr);
        assert_eq!(kv.len(), 0);

        let ptr = kv.as_ptr();
        kv.reserve_exact(0);
        assert_eq!(kv.as_ptr(), ptr);
        assert_eq!(kv.len(), 0);
        kv.reserve_exact(2);
        assert_eq!(kv.as_ptr(), ptr);
        assert_eq!(kv.len(), 0);
    }

    #[test]
    fn next_minimum_capacity() {
        let kv = KVec::new();
        assert_eq!(kv.next_minimum_capacity(0), None);
        assert_eq!(kv.next_minimum_capacity(1), NonZeroUsize::new(1));
        assert_eq!(kv.next_minimum_capacity(3), NonZeroUsize::new(3));
        assert_eq!(kv.next_minimum_capacity(5), NonZeroUsize::new(5));
        assert_eq!(kv.next_minimum_capacity(8), NonZeroUsize::new(8));
        assert_eq!(kv.next_minimum_capacity(10), NonZeroUsize::new(10));
    }

    #[test]
    fn spare_capacity_mut() {
        let mut kv = KVec::new();

        assert_eq!(kv.spare_capacity_mut().len(), 0);

        kv.reserve(32);
        assert_eq!(kv.spare_capacity_mut().len(), 32);

        kv.reserve(33);
        assert_eq!(kv.spare_capacity_mut().len(), 64);

        let mut kv = KVec::new();

        assert_eq!(kv.spare_capacity_mut().len(), 0);

        kv.reserve_exact(32);
        assert_eq!(kv.spare_capacity_mut().len(), 32);

        kv.reserve_exact(33);
        assert_eq!(kv.spare_capacity_mut().len(), 33);

        kv.reserve_exact(20);
        assert_eq!(kv.spare_capacity_mut().len(), 33);
    }

    #[test]
    fn remaining_capacity() {
        let mut kv = KVec::new();
        assert!(kv.capacity() & kv.len() == 0);
        assert_eq!(kv.remaining_capacity(), 0);
        kv.reserve(10);
        assert_eq!(kv.remaining_capacity(), 16);
        kv.reserve(32);
        assert_eq!(kv.remaining_capacity(), 32);
    }

    #[test]
    fn push() {
        let mut kv = KVec::new();

        #[allow(unused_variables)]
        let ptr = kv.as_ptr();
        kv.push("a".into());
        #[cfg(miri)]
        assert_ne!(ptr, kv.as_ptr());
        assert_eq!(kv.len(), 1);

        #[allow(unused_variables)]
        let ptr = kv.as_ptr();
        kv.push("a".into());
        #[cfg(miri)]
        assert_ne!(ptr, kv.as_ptr());
        assert_eq!(kv.len(), 2);

        #[allow(unused_variables)]
        let ptr = kv.as_ptr();
        kv.push("a".into());
        #[cfg(miri)]
        assert_ne!(ptr, kv.as_ptr());
        assert_eq!(kv.len(), 3);
    }

    #[test]
    fn extend() {
        let mut kv = KVec::new();

        kv.extend(["1", "2", "3", "4"].into_iter().map(String::from));
        assert_eq!(kv.len(), 4);
    }

    #[test]
    fn extend_from_slice() {
        let mut kv = KVec::new();
        kv.extend_from_slice(&[String::from("1"), String::from("2"), String::from("3")]);
        assert_eq!(kv.capacity(), 3);
        assert_eq!(kv.len(), 3);
    }

    #[test]
    fn from_iter() {
        let kv = KVec::from_iter([String::from("1"), String::from("2"), String::from("3")]);
        assert_eq!(kv.as_slice(), &["1", "2", "3"]);
        assert_eq!(kv.len(), 3);
        assert_eq!(kv.len(), 3);
    }

    #[test]
    fn remove() {
        let mut kv = KVec::from([String::from("a"), String::from("b")].as_slice());
        let rem = kv.remove(0);

        assert_eq!(rem, "a");
        assert_eq!(kv.as_slice(), &[String::from("b")]);
        assert_eq!(kv.len(), 1);

        let rem = kv.remove(0);
        assert_eq!(rem, "b");
        assert_eq!(kv.as_slice(), &[] as &[&str]);
        assert_eq!(kv.len(), 0);

        let mut kv =
            KVec::from([String::from("a"), String::from("b"), String::from("c")].as_slice());
        let rem = kv.remove(1);
        assert_eq!(rem, "b");
        assert_eq!(kv.as_slice(), &["a", "c"]);
        assert_eq!(kv.len(), 2);

        let rem = kv.remove(0);
        assert_eq!(rem, "a");
        assert_eq!(kv.as_slice(), &[String::from("c")]);
        assert_eq!(kv.len(), 1);

        let rem = kv.remove(0);
        assert_eq!(rem, "c");
        assert_eq!(kv.as_slice(), &[] as &[&str]);
        assert_eq!(kv.len(), 0);
    }

    #[test]
    fn swap_remove() {
        let mut kv =
            KVec::from([String::from("a"), String::from("b"), String::from("c")].as_slice());

        let rem = kv.swap_remove(0);
        assert_eq!(rem, "a");
        assert_eq!(kv.as_slice(), &["c", "b"]);
        assert_eq!(kv.len(), 2);

        let rem = kv.swap_remove(1);
        assert_eq!(rem, "b");
        assert_eq!(kv.as_slice(), &["c"]);
        assert_eq!(kv.len(), 1);

        let rem = kv.swap_remove(0);
        assert_eq!(rem, "c");
        assert_eq!(kv.as_slice(), &[] as &[&str]);
        assert_eq!(kv.len(), 0);
    }

    #[test]
    #[should_panic]
    fn remove_panics() {
        let mut kv =
            KVec::from([String::from("a"), String::from("b"), String::from("c")].as_slice());
        kv.remove(3);
    }

    #[test]
    #[should_panic]
    fn remove_panics_empty() {
        let mut kv = KVec::from([].as_slice());
        kv.remove(0);
    }

    #[test]
    fn is_empty() {
        let mut kv = KVec::new();
        assert!(kv.is_empty());

        kv.push("hi".to_string());
        assert!(!kv.is_empty());

        kv.remove(0);
        assert!(kv.is_empty());
    }

    #[test]
    fn default() {
        let kv = KVec::default();

        assert!(kv.is_empty());
        assert_eq!(kv.capacity(), 0);
    }

    #[test]
    fn into_iter() {
        let kv = KVec::from_iter(["1", "2", "3"].map(String::from));
        let mut kv_iter = kv.into_iter();
        assert_eq!(kv_iter.next().unwrap(), "1");
        assert_eq!(kv_iter.next_back().unwrap(), "3");
        assert_eq!(kv_iter.next_back().unwrap(), "2");
        assert_eq!(kv_iter.next(), None);
        assert_eq!(kv_iter.next_back(), None);
        assert_eq!(kv_iter.next(), None);
        assert_eq!(kv_iter.next_back(), None);
    }

    #[test]
    fn into_iter_empty() {
        let kv = KVec::from_iter([] as [String; 0]);
        let mut kv_iter = kv.into_iter();
        assert_eq!(kv_iter.next(), None);
        assert_eq!(kv_iter.next_back(), None);
        assert_eq!(kv_iter.next(), None);
        assert_eq!(kv_iter.next_back(), None);
    }

    #[test]
    fn truncate() {
        let mut kv = KVec::from_iter(["1", "2", "3", "4"].map(String::from));
        let cap = kv.capacity();
        kv.truncate(2);
        assert_eq!(kv.len(), 2);
        assert_eq!(kv.capacity(), cap);
        kv.truncate(3);
        assert_eq!(kv.len(), 2);
        assert_eq!(kv.capacity(), cap);
        kv.truncate(30);
        assert_eq!(kv.len(), 2);
        assert_eq!(kv.capacity(), cap);
    }
}
