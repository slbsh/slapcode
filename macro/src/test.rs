use crate::Token::*;
use crate::process;

#[test]
fn soundness_check() {
    let mut map = std::collections::HashMap::new();
    map.insert("a", vec![ThingA, ThingC, ThingB ]);
    map.insert("b", vec![ThingC, ThingB, ThingB ]);
    map.insert("c", vec![ThingA, ThingA, ThingC ]);

    process(&mut map);

    assert_eq!(map.get("a").unwrap(), &[ThingA, ThingC, ThingB]);
    assert_eq!(map.get("b").unwrap(), &[ThingC, ThingB, ThingB]);
    assert_eq!(map.get("c").unwrap(), &[ThingA, ThingA, ThingC]);
}

#[test]
fn macro_expansion_simple() {
    let mut map = std::collections::HashMap::new();
    map.insert("a", vec![ThingA,     ThingC, Macro("c")]);
    map.insert("b", vec![ThingC,     ThingB, ThingB    ]);
    map.insert("c", vec![Macro("b"), ThingA, ThingC    ]);
    
    process(&mut map);

    assert_eq!(map.get("a").unwrap(), &[ThingA, ThingC, ThingC, ThingB, ThingB, ThingA, ThingC]);
    assert_eq!(map.get("b").unwrap(), &[ThingC, ThingB, ThingB]);
    assert_eq!(map.get("c").unwrap(), &[ThingC, ThingB, ThingB, ThingA, ThingC]);
}


#[test]
fn macro_expansion_simple_var() {
    let mut map = std::collections::HashMap::new();
    map.insert("b", vec![ThingC,     ThingB, ThingB    ]);
    map.insert("c", vec![Macro("b"), ThingA, ThingC    ]);
    map.insert("a", vec![ThingA,     ThingC, Macro("c")]);
    
    process(&mut map);

    assert_eq!(map.get("a").unwrap(), &[ThingA, ThingC, ThingC, ThingB, ThingB, ThingA, ThingC]);
    assert_eq!(map.get("b").unwrap(), &[ThingC, ThingB, ThingB]);
    assert_eq!(map.get("c").unwrap(), &[ThingC, ThingB, ThingB, ThingA, ThingC]);
}

#[test]
fn macro_expansion_recursive() {
    let mut map = std::collections::HashMap::new();
    map.insert("a", vec![ThingA,     ThingC, Macro("a")]);
    map.insert("b", vec![ThingC,     ThingB, ThingB    ]);
    map.insert("c", vec![Macro("b"), ThingA, Macro("a")]);
    
    process(&mut map);

    assert_eq!(map.get("a").unwrap(), &[ThingA, ThingC]);
    assert_eq!(map.get("b").unwrap(), &[ThingC, ThingB, ThingB]);
    assert_eq!(map.get("c").unwrap(), &[ThingC, ThingB, ThingB, ThingA, ThingA, ThingC]);
}

#[test]
fn macro_expansion_redifine() {
    let mut map = std::collections::HashMap::new();
    map.insert("a", vec![ThingA,     ThingC]);
    map.insert("b", vec![ThingC,     Macro("a"), ThingB]);
    map.insert("a", vec![ThingC,     Macro("a")]);
    map.insert("c", vec![Macro("b"), ThingA, Macro("a")]);
    
    process(&mut map);

    assert_eq!(map.get("a").unwrap(), &[ThingC, ThingA, ThingC]);
    assert_eq!(map.get("b").unwrap(), &[ThingC, ThingC, ThingA, ThingC, ThingB]);
    assert_eq!(map.get("c").unwrap(), &[ThingC, ThingC, ThingA, ThingC, ThingB, ThingA, ThingC, ThingA, ThingC]);
}
