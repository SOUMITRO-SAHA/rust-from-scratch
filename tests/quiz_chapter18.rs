use rust::chapter18::quiz::*;

#[test]
fn test_match_enum() {
    assert_eq!(match_enum(MyEnum::A), "A");
    assert_eq!(match_enum(MyEnum::B(7)), "B(7)");
    assert_eq!(match_enum(MyEnum::C("hi".to_string())), "C(hi)");
}

#[test]
fn test_if_while_let() {
    assert_eq!(if_while_let(vec![Some(1), Some(2), None, Some(3)]), 3);
}

#[test]
fn test_match_struct() {
    assert_eq!(match_struct(Point { x: 3, y: 0 }), 3);
    assert_eq!(match_struct(Point { x: 0, y: 4 }), 4);
    assert_eq!(match_struct(Point { x: 2, y: 2 }), 4);
}

#[test]
fn test_match_guard() {
    assert_eq!(match_guard(-5), "negative");
    assert_eq!(match_guard(0), "zero");
    assert_eq!(match_guard(8), "positive");
}

#[test]
fn test_match_tuple() {
    assert!(match_tuple((0, 9)));
    assert!(!match_tuple((1, 0)));
}
