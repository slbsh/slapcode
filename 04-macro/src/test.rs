use crate::process;
use crate::Key;
use crate::Token::*;

#[test]
fn soundness_check() {
    let mut map = std::collections::HashMap::new();
    map.insert("a".into(), vec![ThingA, ThingC, ThingB]);
    map.insert("b".into(), vec![ThingC, ThingB, ThingB]);
    map.insert("c".into(), vec![ThingA, ThingA, ThingC]);

    let map = process(map);

    assert_eq!(map.get("a").unwrap(), &[ThingA, ThingC, ThingB]);
    assert_eq!(map.get("b").unwrap(), &[ThingC, ThingB, ThingB]);
    assert_eq!(map.get("c").unwrap(), &[ThingA, ThingA, ThingC]);
}

#[test]
fn macro_expansion_simple() {
    let mut map = std::collections::HashMap::new();
    map.insert("a".into(), vec![ThingA,     ThingC, Macro("c")]);
    map.insert("b".into(), vec![ThingC,     ThingB, ThingB]);
    map.insert("c".into(), vec![Macro("b"), ThingA, ThingC]);

    let map = process(map);

    assert_eq!(map.get("a").unwrap(), &[ThingA, ThingC, ThingC, ThingB, ThingB, ThingA, ThingC]);
    assert_eq!(map.get("b").unwrap(), &[ThingC, ThingB, ThingB]);
    assert_eq!(map.get("c").unwrap(), &[ThingC, ThingB, ThingB, ThingA, ThingC]);
}

#[test]
fn macro_expansion_simple_var() {
    let mut map = std::collections::HashMap::new();
    map.insert("b".into(), vec![ThingC,     ThingB, ThingB]);
    map.insert("c".into(), vec![Macro("b"), ThingA, ThingC]);
    map.insert("a".into(), vec![ThingA,     ThingC, Macro("c")]);

    let map = process(map);

    assert_eq!(map.get("a").unwrap(), &[ThingA, ThingC, ThingC, ThingB, ThingB, ThingA, ThingC]);
    assert_eq!(map.get("b").unwrap(), &[ThingC, ThingB, ThingB]);
    assert_eq!(map.get("c").unwrap(), &[ThingC, ThingB, ThingB, ThingA, ThingC]);
}

#[test]
fn macro_expansion_recursive() {
    let mut map = std::collections::HashMap::new();
    map.insert("a".into(), vec![ThingA,     ThingC, Macro("a")]);
    map.insert("b".into(), vec![ThingC,     ThingB, ThingB]);
    map.insert("c".into(), vec![Macro("b"), ThingA, Macro("a")]);

    let map = process(map);

    assert_eq!(map.get("a").unwrap(), &[ThingA, ThingC]);
    assert_eq!(map.get("b").unwrap(), &[ThingC, ThingB, ThingB]);
    assert_eq!(map.get("c").unwrap(), &[ThingC, ThingB, ThingB, ThingA, ThingA, ThingC]);
}

#[test]
fn macro_expansion_redifine() {
    let mut map = std::collections::HashMap::new();
    map.insert("a".into(),                    vec![ThingA,     ThingC]);
    map.insert("b".into(),                    vec![ThingC,     Macro("a"), ThingB]);
    map.insert(Key::new_with_version("a", 1), vec![ThingC,     Macro("a")]);
    map.insert("c".into(),                    vec![Macro("b"), ThingA,     Macro("a")]);

    let map = process(map);

    assert_eq!(map.get("a").unwrap(), &[ThingC, ThingA, ThingC]);
    assert_eq!(map.get("b").unwrap(), &[ThingC, ThingC, ThingA, ThingC, ThingB]);
    assert_eq!(map.get("c").unwrap(), &[ThingC, ThingC, ThingA, ThingC, ThingB, ThingA, ThingC, ThingA, ThingC]);
}
