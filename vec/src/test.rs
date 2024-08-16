use crate::Vec;

#[test]
fn len_basic() {
    let mut vec: Vec<u64> = Vec::new();
    (0..10).for_each(|i| vec.push(i));

    assert_eq!(vec.len(), 10);
}

#[test]
fn push_basic() {
    let mut vec: Vec<u64> = Vec::new();
    (0..10).for_each(|i| vec.push(i));

    assert_eq!(vec.as_slice(), &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn get_basic() {
    let mut vec: Vec<u64> = Vec::new();
    (0..10).for_each(|i| vec.push(i));

    assert_eq!(vec.get(0), Some(&0));
    assert_eq!(vec.get(1), Some(&1));
    assert_eq!(vec.get(2), Some(&2));
    assert_eq!(vec.get(3), Some(&3));
    assert_eq!(vec.get(4), Some(&4));
    assert_eq!(vec.get(5), Some(&5));
    assert_eq!(vec.get(6), Some(&6));
    assert_eq!(vec.get(7), Some(&7));
    assert_eq!(vec.get(8), Some(&8));
    assert_eq!(vec.get(9), Some(&9));
}

#[test]
fn get_out_of_bounds() {
    let mut vec: Vec<u64> = Vec::new();
    (0..10).for_each(|i| vec.push(i));

    assert_eq!(vec.get(8), Some(&8));
    assert_eq!(vec.get(9), Some(&9));
    assert_eq!(vec.get(10), None);
    assert_eq!(vec.get(11), None);
}

use rand::Rng;

#[test]
fn from_slice_rand() {
    let mut rng = rand::thread_rng();
    let mut random_numbers = [0; 100];
    
    for num in &mut random_numbers {
        *num = rng.gen();
    }

    let vec = Vec::from_slice(&random_numbers);

    assert_eq!(vec.as_slice(), &random_numbers);
}

#[test]
fn push_pop_rand() {
    let mut rng = rand::thread_rng();
    let mut random_numbers = [0; 100];
    let mut vec: Vec<u64> = Vec::new();
    
    for num in &mut random_numbers {
        let rand_num: u64 = rng.gen();
        *num = rand_num;
        vec.push(rand_num);
    }

    for _ in 0..40 {
        vec.pop();
    }

    assert_eq!(vec.as_slice(), &random_numbers[0..60]);
}
