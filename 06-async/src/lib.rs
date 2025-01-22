#![deny(clippy::all)]
#![allow(clippy::option_map_unit_fn, clippy::wrong_self_convention, clippy::uninit_assumed_init, uncommon_codepoints, non_camel_case_types)]

use std::future::Future;
use std::sync::{Arc, Mutex, Condvar};
use std::task::{Context, Poll, Waker};

fn create_waker() -> Waker {
	todo!()
}

// should run until completion
pub fn spawn_blocking<F: Future<Output=()> + Send + 'static>(f: F) {
	todo!()
}

#[cfg(test)]
mod test;
