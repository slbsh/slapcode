use crate::do_thing;

#[test]
fn one() {
    let vec = vec![7, 1, 5, 4];
    assert_eq!(do_thing(vec), 4);
}

#[test]
fn two() {
    let vec = vec![9, 4, 3, 2;
    assert_eq!(do_thing(vec), -1);
}

#[test]
fn three() {
    let vec = vec![1, 5, 2, 10];
    assert_eq!(do_thing(vec), 9);
}
