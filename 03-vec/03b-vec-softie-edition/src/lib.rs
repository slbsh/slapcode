#![no_std] // hehe >:D
#![allow(forbidden_lint_groups)]
#![forbid(clippy::all)]
#![allow(clippy::option_map_unit_fn, clippy::wrong_self_convention, clippy::uninit_assumed_init, uncommon_codepoints, non_camel_case_types, clippy::new_without_default, clippy::len_without_is_empty)]

use core::*;
use libc::*; // malloc, realloc, free ;)

pub struct Vec<λ> {
    ptr: *mut λ,
    len: usize,
    cap: usize,
}

impl<λ> Vec<λ> {
    pub fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }

    fn realloc(&mut self) {
        todo!("realloc")
    }

    pub fn push(&mut self, value: λ) {
        todo!("push")
    }

    pub fn pop(&mut self) -> Option<λ> {
        todo!("pop")
    }

    pub fn get(&self, index: λ) -> Option<&λ> {
        todo!("get")
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[λ] {
        match self.ptr.is_null() {
            true => &[],
            false => unsafe {
                core::slice::from_raw_parts(self.ptr, self.len)
            },
        }
    }
}

impl<λ: Clone> Vec<λ> {
    pub fn from_slice(slice: &[λ]) -> Self {
        todo!("from_slice")
    }

}

impl<λ> Drop for Vec<λ> {
    fn drop(&mut self) {
        todo!("drop")
    }
}

#[cfg(test)]
mod test;
