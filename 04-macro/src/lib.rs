#![allow(forbidden_lint_groups)]
#![forbid(clippy::all)]
#![allow(clippy::option_map_unit_fn, clippy::wrong_self_convention, clippy::uninit_assumed_init, uncommon_codepoints, non_camel_case_types, clippy::new_without_default, clippy::len_without_is_empty)]

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Key {
    ident: &'static str,
    version: usize,
}

impl From<&'static str> for Key {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

impl Key {
    pub fn new(ident: &'static str) -> Self {
        Self::new_with_version(ident, 0)
    }

    pub fn new_with_version(ident: &'static str, version: usize) -> Self {
        Self { ident, version }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Macro(&'a str),
    ThingA,
    ThingB,
    ThingC,
}

// this is what I've implemented in the preprocessor already for somethin else,
// so why not just use it eh? ;)
impl<'a> Token<'a> {
    fn try_as_macro(&self) -> Option<&'a str> {
        match self {
            Token::Macro(m) => Some(m),
            _ => None,
        }
    }
}

pub fn process(map: HashMap<Key, Vec<Token>>) -> HashMap<&str, Vec<Token>> {
    todo!("do the thing")
}

#[cfg(test)]
mod test;
