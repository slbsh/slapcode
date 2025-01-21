#![deny(clippy::all)]
#![allow(clippy::option_map_unit_fn, clippy::wrong_self_convention, clippy::uninit_assumed_init, uncommon_codepoints, non_camel_case_types)]

use std::pin::Pin;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

pub struct Runtime {
	task:   Arc<Mutex<Pin<Option<Box<dyn Future<Output = ()> + Send>>>>>,
	waker:  Arc<Waker>,
	/* you may add fields */
}

impl Runtime {
	fn create_waker() -> Waker {
		todo!()
	}

	// should run until completion
	pub fn spawn_blocking<F: Future<Output=()> + Send + 'static>(f: F) {
		todo!()
	}
}

#[cfg(test)]
mod test;
