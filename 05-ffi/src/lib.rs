#![allow(forbidden_lint_groups)]
#![forbid(clippy::all)]
#![allow(clippy::option_map_unit_fn, clippy::wrong_self_convention, clippy::uninit_assumed_init, uncommon_codepoints, non_camel_case_types)]

#[repr(C)]
struct Opt {
    has_value: bool,
    value: i32,
}

#[repr(C)]
struct Stack {
    /* add fields? */
}

extern "C" {
    /* add funcs */
}

impl Stack {
    fn new() -> Self {
        todo!("do new")
    }

    fn len(&self) -> usize {
        todo!("do len")
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn pop(&mut self) -> Option<i32> {
        todo!("do pop")
    }

    fn push(&mut self, value: i32) {
        todo!("do push")
    }

    fn peek(&self) -> Option<i32> {
        todo!("do peek")
    }

    // impl it here.. or in cpp if you're brave
    fn sort(&mut self) {
        todo!("do sort")
    }
}

impl Clone for Stack {
    // hint: std::slice::from_raw_parts
    fn clone(&self) -> Self {
        todo!("do clone")
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        todo!("do drop")
    }
}

use std::fmt;
impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stack = self.clone();
        write!(f, "[")?;
        while let Some(value) = stack.pop() {
            write!(f, "{}", value)?;
            if stack.len() > 0 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test;
