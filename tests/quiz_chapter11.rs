use rust::chapter11::quiz::*;

#[test]
fn test_double() {
    assert_eq!(double(2), 4);
    assert_eq!(double(-3), -6);
}

#[test]
#[should_panic]
fn test_will_panic() {
    will_panic();
}

#[test]
fn test_greet() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
    assert_eq!(greet(""), "Hello, !");
}
