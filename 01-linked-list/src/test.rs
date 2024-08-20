use crate::List;
use rand::{Rng, SeedableRng};
use std::fmt::{self, Debug};

impl<T: Debug> Debug for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let mut node = self.0;
            write!(f, "[")?;
            while !node.is_null() {
                write!(f, "{:?}", (*node).data)?;
                node = (*node).next;
                if !node.is_null() {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")
        }
    }
}

#[test]
fn from_basic_int() {
    let vec = (0..8).collect::<Vec<_>>();
    let list = List::from(vec.clone());
    assert_eq!(format!("{:?}", list), format!("{:?}", vec));
}

#[test]
fn from_rand_int() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let vec = (0..100).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();

    let list = List::from(vec.clone());
    assert_eq!(format!("{:?}", list), format!("{:?}", vec));
}

#[test]
fn from_rand_str() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let vec = (0..100)
        .map(|_| {
            (0..rng.gen_range(1..=10))
                .map(|_| rng.gen::<char>())
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    let list = List::from(vec.clone());
    assert_eq!(format!("{:?}", list), format!("{:?}", vec));
}

#[test]
fn sort_basic_int() {
    let mut vec = vec![3, 8, 6, 2, 1, 4, 5, 7];
    let mut list = List::from(vec.clone());

    list.sort();
    vec.sort();
    assert_eq!(format!("{:?}", list), format!("{:?}", vec));
}

#[test]
fn sort_rand_int() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut vec = (0..100).map(|_| rng.gen::<u64>()).collect::<Vec<_>>();
    let mut list = List::from(vec.clone());

    list.sort();
    vec.sort();
    assert_eq!(format!("{:?}", list), format!("{:?}", vec));
}

#[test]
fn sort_rand_str() {
    let mut rng = rand::rngs::SmallRng::from_entropy();
    let mut vec = (0..100)
        .map(|_| {
            (0..rng.gen_range(1..=10))
                .map(|_| rng.gen::<char>())
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    let mut list = List::from(vec.clone());

    list.sort();
    vec.sort();
    assert_eq!(format!("{:?}", list), format!("{:?}", vec));
}
