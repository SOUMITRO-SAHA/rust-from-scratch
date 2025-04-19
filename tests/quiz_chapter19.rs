use rust::chapter19::quiz::*;

#[test]
fn test_myiter() {
    let mut c = Counter { count: 0 };
    assert_eq!(c.next(), Some(1));
    assert_eq!(c.next(), Some(2));
    assert_eq!(c.next(), Some(3));
    assert_eq!(c.next(), None);
}

#[test]
fn test_point_add() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    assert_eq!(p3.x, 4);
    assert_eq!(p3.y, 6);
}

#[test]
fn test_call_trait_method() {
    let b = Bar;
    assert_eq!(call_trait_method(&b), "foo");
}

#[test]
fn test_supertrait() {
    let ab = AB;
    assert_eq!(ab.a(), 1);
    assert_eq!(ab.b(), 2);
}

#[test]
fn test_newtype_wrapper() {
    let w = Wrapper(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(format!("{}", w), "[a, b]");
}
