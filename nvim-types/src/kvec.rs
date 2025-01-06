use std::mem::{self, MaybeUninit};
use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut, RangeBounds};
use std::ptr::NonNull;

use panics::{alloc_failed, slice_error};

#[repr(C)]
#[derive(Debug)]
pub struct KVec<T> {
    pub(super) len: usize,
    pub(super) capacity: usize,
    pub(super) ptr: NonNull<T>,
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
            ptr: NonNull::dangling(),
        }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        self.slice_check();
        // since self.ptr is non null, a slice is always valid since [`NonNull::dangling`] is
        // correctly aligned.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        self.slice_check();
        // since self.ptr is non null, a slice is always valid since [`NonNull::dangling`] is
        // correctly aligned.
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }

    /// Returns the uninitialized but allocated space for `T`
    #[inline]
    pub const fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] {
        self.slice_check();
        unsafe {
            std::slice::from_raw_parts_mut(
                self.ptr.add(self.len).cast::<MaybeUninit<T>>().as_ptr(),
                self.capacity - self.len(),
            )
        }
    }

    /// Returns a pointer to the internal buffer
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
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

    /// Set the length of the [`KVec`]
    ///
    /// This should usually called after initializing elements via [`KVec::spare_capacity_mut`].
    ///
    /// # Safety
    ///
    /// Calling this function when len items are not initialized may cause undefined behavior.
    #[inline(always)]
    pub const unsafe fn set_len(&mut self, len: usize) {
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
        let ptr = if Self::ZST || capacity == 0 {
            NonNull::dangling()
        } else {
            let Some(byte_cap) = capacity.checked_mul(Self::T_SIZE) else {
                alloc_failed();
            };

            // When the requested size is 0, malloc may return NULL or a dangling pointer.
            // To always have a non null pointer we check the capacity above.
            debug_assert_ne!(byte_cap, 0);

            let ptr = unsafe { libc::malloc(byte_cap) };
            if ptr.is_null() {
                alloc_failed();
            }
            unsafe { NonNull::new_unchecked(ptr as *mut T) }
        };

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
        debug_assert!(self.capacity() >= self.len());
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
        // SAFETY: additional is always bigger than remaining which is checked above
        let new_capacity = self.capacity + additional - remaining;
        debug_assert_ne!(new_capacity, 0);
        unsafe { Some(NonZeroUsize::new_unchecked(new_capacity)) }
    }

    // Inlined as most of the conditions are likely to be checked in call sites.
    #[inline(always)]
    fn realloc(&mut self, new_capacity: usize) {
        if Self::ZST {
        } else if new_capacity == 0 && self.capacity > 0 {
            unsafe { libc::free(self.ptr.as_ptr() as *mut libc::c_void) };
        } else {
            let Some(byte_capacity) = new_capacity.checked_mul(Self::T_SIZE) else {
                alloc_failed();
            };
            let ptr = if self.capacity == 0 {
                unsafe { libc::malloc(byte_capacity) }
            } else {
                unsafe { libc::realloc(self.ptr.as_ptr() as *mut libc::c_void, byte_capacity) }
            };
            if ptr.is_null() {
                alloc_failed();
            }
            self.ptr = unsafe { NonNull::new_unchecked(ptr as *mut T) };
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
    unsafe fn push_unchecked(&mut self, element: T) {
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
            let spare = unsafe { self.spare_capacity_mut().get_unchecked_mut(..s.len()) };
            for i in 0..s.len() {
                spare[i].write(s[i].clone());
            }
        }

        unsafe { self.set_len(self.len() + s.len()) };
    }

    /// Drain the elements in range
    ///
    /// Similar to [`Vec::drain`], this returns an iterator that yields the items in range.
    ///
    /// # Panics
    ///
    /// If the range exceeds the bounds of the [`KVec`] this will panic.
    fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> Drain<'_, T> {
        let std::ops::Range { start, end } =
            range_bound_to_range(self, range).expect("range bounds must never be out of bounds");

        unsafe {
            Drain {
                start,
                end,
                kvec: NonNull::new_unchecked(self as *mut _),
                iter: self.as_slice()[start..end].iter(),
            }
        }
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

#[inline]
fn range_bound_to_range<T, R: RangeBounds<usize>>(
    kv: &KVec<T>,
    r: R,
) -> Option<std::ops::Range<usize>> {
    use std::ops::Bound;
    let start = match r.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(i) => *i,
        Bound::Excluded(i) => i.saturating_add(1),
    };
    let end = match r.end_bound() {
        Bound::Unbounded => kv.len,
        Bound::Included(i) => *i,
        Bound::Excluded(i) => i.saturating_sub(1),
    };
    if (start < kv.len()) || (start <= end) || (end < kv.len()) {
        return Some(start..end);
    }
    None
}

impl<T> Drop for KVec<T> {
    fn drop(&mut self) {
        if Self::ZST || self.capacity == 0 {
            return;
        }
        let ptr = self.as_ptr();
        for i in 0..self.len() {
            unsafe { std::ptr::drop_in_place(ptr.add(i)) }
        }
        unsafe {
            libc::free(self.as_ptr() as *mut libc::c_void);
        }
    }
}

/// TODO: probably safety problems, check std implementation
struct Drain<'a, T> {
    start: usize,
    end: usize,
    iter: std::slice::Iter<'a, T>,
    kvec: NonNull<KVec<T>>,
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|elt| unsafe { std::ptr::read(elt as *const T) })
    }
}

impl<T> Drop for Drain<'_, T> {
    fn drop(&mut self) {
        // add ZST check
        unsafe {
            let kvec = self.kvec.as_mut();
            for elt in
                kvec.as_mut_slice()[self.start..self.end.min(self.iter.as_slice().len())].iter_mut()
            {
                std::ptr::drop_in_place(elt as *mut T);
            }
            let src = kvec.ptr.add(self.start);
            let dst = kvec.ptr.add(self.end);
            libc::memcpy(
                dst.as_ptr() as *mut libc::c_void,
                src.as_ptr() as *const libc::c_void,
                (self.end - self.start) * mem::size_of::<T>(),
            );
            kvec.set_len(kvec.len() - (self.end - self.start));
        }
    }
}

const _: () = assert!(
    mem::size_of::<KVec<[u32; 1]>>()
        == mem::size_of::<usize>() * 2 + mem::size_of::<*mut [u32; 10]>()
);

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    type KVec = super::KVec<String>;

    #[test]
    fn new() {
        let kv = KVec::new();
        assert_eq!(kv.len(), 0);
        assert_eq!(kv.capacity(), 0);
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
        // the pointer should also change since at this stage it should be a dangling pointer
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
}
