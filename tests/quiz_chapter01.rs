// Quiz test for Chapter 1
use rust::chapter01::quiz;

#[test]
fn test_hello_world() {
    assert_eq!(quiz::hello_world(), "Hello, world!");
}

#[test]
fn test_print_learning_rust() {
    // Just check that the function runs without panicking
    quiz::print_learning_rust();
    // If you want to check output, use integration tests with assert_cmd or similar.
}
