#![no_std] // hehe >:D
#![allow(forbidden_lint_groups)]
#![forbid(clippy::all)]
#![allow(clippy::option_map_unit_fn, clippy::wrong_self_convention, clippy::uninit_assumed_init, uncommon_codepoints, non_camel_case_types)]

// preferably use only these imports
use core::{ptr, mem};
use libc::*;

#[repr(C)]
struct Vec<λ> { // dont change this
    start: *mut λ,
    last:  *mut λ,
    end:   *mut λ,
}

impl<λ> Vec<λ> {
    pub fn new() -> Self {
        Self {
            start: ptr::null_mut(),
            last:  ptr::null_mut(),
            end:   ptr::null_mut(),
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

    pub fn len(&self) -> usize {
        todo!("len")
    }

    pub fn as_slice(&self) -> &[λ] {
        match self.start.is_null() {
            true => &[],
            false => unsafe {
                core::slice::from_raw_parts(self.start, self.len())
            }
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
