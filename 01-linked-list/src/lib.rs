#![allow(forbidden_lint_groups)]
#![forbid(clippy::all)]
#![allow(clippy::option_map_unit_fn, clippy::wrong_self_convention, clippy::uninit_assumed_init, clippy::new_without_default, non_camel_case_types)]

#[repr(transparent)]
pub struct List<λ>(*mut Node<λ>); // power of lambda!

pub struct Node<λ> {
    next: *mut Node<λ>,
    data: λ,
}

impl<λ> From<Vec<λ>> for List<λ> {
    fn from(v: Vec<λ>) -> Self { 
        todo!("do the from")
    }
}


impl<λ: Ord> List<λ> {
    pub fn sort(&mut self) {
        todo!("do the sort")
    }
}


#[cfg(test)]
mod test;
