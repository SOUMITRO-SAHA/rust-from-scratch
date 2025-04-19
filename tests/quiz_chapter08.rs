use rust::chapter08::quiz::*;

#[test]
fn test_sum_vector() {
    assert_eq!(sum_vector(), 6); // 1 + 2 + 3
}

#[test]
fn test_append_strings() {
    assert_eq!(append_strings("foo", "bar"), "foobar");
    assert_eq!(append_strings("", "baz"), "baz");
}

#[test]
fn test_get_score() {
    assert_eq!(get_score(), 10);
}
