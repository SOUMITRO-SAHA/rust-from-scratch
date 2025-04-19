use rust::chapter10::quiz::*;

#[test]
fn test_largest() {
    assert_eq!(*largest(&[1, 2, 3]), 3);
    assert_eq!(*largest(&['a', 'z', 'm']), 'z');
}

#[test]
fn test_describable_trait() {
    let a = Animal { name: String::from("dog") };
    assert_eq!(a.describe(), "This is a dog.");
}

#[test]
fn test_longer() {
    assert_eq!(longer("foo", "foobar"), "foobar");
    assert_eq!(longer("hello", "hi"), "hello");
}
