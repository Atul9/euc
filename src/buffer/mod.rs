#[cfg(feature = "no_std")]
use alloc::prelude::*;

use crate::Target;

pub struct Buffer2d<T> {
    items: Vec<T>,
    size: [usize; 2],
}

impl<T: Clone> Buffer2d<T> {
    pub fn new(size: [usize; 2], fill: T) -> Self {
        Self {
            items: vec![fill; size[0] * size[1]],
            size,
        }
    }
}

impl<T: Clone> Target for Buffer2d<T> {
    type Item = T;

    #[inline(always)]
    fn size(&self) -> [usize; 2] {
        self.size
    }

    #[inline(always)]
    unsafe fn set(&mut self, pos: [usize; 2], item: Self::Item) {
        *self.items.get_unchecked_mut(pos[1] * self.size[0] + pos[0]) = item;
    }

    #[inline(always)]
    unsafe fn get(&self, pos: [usize; 2]) -> &Self::Item {
        &self.items.get_unchecked(pos[1] * self.size[0] + pos[0])
    }

    fn clear(&mut self, fill: Self::Item) {
        self.items = vec![fill; self.size[0] * self.size[1]];
    }
}

impl<T> AsRef<[T]> for Buffer2d<T> {
    fn as_ref(&self) -> &[T] {
        &self.items
    }
}
