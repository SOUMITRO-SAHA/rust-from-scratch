// Quiz test for Chapter 2
use rust::chapter02::quiz;

#[test]
fn test_mutable_var() {
    assert_eq!(quiz::mutable_var(), 10);
}

#[test]
fn test_parse_42() {
    assert_eq!(quiz::parse_42(), 42);
}

#[test]
fn test_string_length() {
    assert_eq!(quiz::string_length("rust"), 4);
    assert_eq!(quiz::string_length("") , 0);
    assert_eq!(quiz::string_length("hello world"), 11);
}
