#![allow(forbidden_lint_groups)]
#![forbid(clippy::all)]
#![allow(clippy::option_map_unit_fn, clippy::wrong_self_convention, clippy::uninit_assumed_init, uncommon_codepoints, non_camel_case_types)]

use std::pin::Pin;
use std::future::Future;
use std::any::Any;

use std::task::{Context, Poll};

struct Task(Pin<Box<dyn Future<Output = Box<dyn Any + Send>> + Send>>);

impl Future for Task {
    type Output = Box<dyn Any + Send>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.0.as_mut().poll(cx)
    }
}


struct Executor(VecDeque<Task>);

impl Executor {
    fn new() -> Self {
        Executor(VecDeque::new())
    }

    fn spawn(&mut self, task: impl Future<Output = Box<dyn Any + Send>> + Send + 'static) {
        self.0.push_back(Task(Box::pin(task)));
    }
}


#[cfg(test)]
mod test;
