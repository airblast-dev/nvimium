use core::slice;
use std::iter::{FusedIterator, Map};

use super::{Object, ThinString};

pub struct ThIter<'a> {
    inner: Map<slice::Iter<'a, Object>, fn(&'a Object) -> ThinString<'a>>,
}

impl<'a> ThIter<'a> {
    pub(crate) fn new(s: &'a [Object]) -> Self {
        Self {
            inner: s.iter().map(|obj| obj.as_string().unwrap().as_thinstr()),
        }
    }
}

impl<'a> Iterator for ThIter<'a> {
    type Item = ThinString<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.inner.count()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.inner.last()
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n)
    }
}

impl<'a> DoubleEndedIterator for ThIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth_back(n)
    }
}

impl<'a> ExactSizeIterator for ThIter<'a> {}
impl<'a> FusedIterator for ThIter<'a> {}
