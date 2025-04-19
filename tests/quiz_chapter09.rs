use rust::chapter09::quiz::*;

#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10, 2), Ok(5));
    assert_eq!(safe_divide(1, 0), Err("Cannot divide by zero".to_string()));
}

#[test]
fn test_check_not_empty() {
    assert_eq!(check_not_empty("foo"), Ok("foo"));
    assert_eq!(check_not_empty(""), Err("Empty string".to_string()));
}

#[test]
fn test_read_file_quiz() {
    // This test will pass if Err is returned (since there's no real file)
    let result = read_file_quiz("no_such_file.txt");
    assert!(result.is_err());
}
