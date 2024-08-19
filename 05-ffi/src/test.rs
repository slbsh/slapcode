use std::mem;
use crate::Stack;
use rand::{SeedableRng, Rng};

#[test]
fn construct_drop() {
    let stack = Stack::new();
    assert_eq!(stack.len(), 0);
    assert!(stack.is_empty());
    mem::drop(stack);
}

#[test]
fn push_pop() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.is_empty(), false);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.is_empty(), true);
}

#[test]
fn push_pop_rand() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut stack = Stack::new();
    let mut vec = Vec::new();

    (0..100).for_each(|_| {
        let value: i32 = rng.gen();
        stack.push(value);
        vec.push(value);
    });

    let vec = vec.into_iter().rev().collect::<Vec<_>>();

    assert_eq!(stack.len(), vec.len());
    assert_eq!(format!("{:?}", stack), format!("{:?}", vec));
}

#[test]
fn sort() {
    let mut stack = Stack::new();
    stack.push(3);
    stack.push(1);
    stack.push(2);
    stack.sort();
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(3));
}

#[test]
fn sort_rand() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut stack = Stack::new();
    let mut vec = Vec::new();

    (0..100).for_each(|_| {
        let value: i32 = rng.gen();
        stack.push(value);
        vec.push(value);
    });

    vec.sort();

    stack.sort();

    assert_eq!(stack.len(), vec.len());
    assert_eq!(format!("{:?}", stack), format!("{:?}", vec));
}

#[test]
fn clone() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut stack = Stack::new();

    (0..100).for_each(|_| stack.push(rng.gen()));

    let stack2 = stack.clone();
    assert_eq!(format!("{:?}", stack), format!("{:?}", stack2));
}

#[test]
fn peek_rand() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut stack = Stack::new();

    (0..100).for_each(|_| stack.push(rng.gen()));

    let mut stack2 = stack.clone();

    for _ in 0..100 {
        assert_eq!(stack.peek(), stack2.pop());
        stack.pop();
    }
}
