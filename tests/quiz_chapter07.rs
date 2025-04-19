use rust::chapter07::quiz::*;

#[test]
fn test_math_add() {
    assert_eq!(math::add(2, 3), 5);
    assert_eq!(math::add(-1, 1), 0);
}

#[test]
fn test_call_add() {
    assert_eq!(call_add(10, 20), 30);
}

#[test]
fn test_nested_module_hello() {
    assert_eq!(outer::inner::hello(), "Hello from inner!");
    assert_eq!(call_hello(), "Hello from inner!");
}
