use rust::chapter04::quiz;

#[test]
fn test_take_and_len() {
    assert_eq!(quiz::take_and_len(String::from("hello")), 5);
    assert_eq!(quiz::take_and_len(String::from("")), 0);
}

#[test]
fn test_borrow_and_len() {
    let s = String::from("rustacean");
    assert_eq!(quiz::borrow_and_len(&s), 9);
}

#[test]
fn test_first_word() {
    assert_eq!(quiz::first_word("hello world"), "hello");
    assert_eq!(quiz::first_word("rust"), "rust");
    assert_eq!(quiz::first_word(""), "");
}
