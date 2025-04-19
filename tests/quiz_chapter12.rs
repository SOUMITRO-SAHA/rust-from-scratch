use rust::chapter12::quiz::*;

#[test]
fn test_arg_count() {
    // Can't reliably test actual CLI args, but should be >= 1 (the program name)
    assert!(arg_count() >= 1);
}

#[test]
fn test_read_file() {
    // Should return Err for missing file
    let result = read_file("no_such_file.txt");
    assert!(result.is_err());
}

#[test]
fn test_search_lines() {
    let text = "foo\nbar\nfoo bar\nbaz";
    assert_eq!(search_lines(text, "foo"), vec![1, 3]);
    assert_eq!(search_lines(text, "baz"), vec![4]);
    assert_eq!(search_lines(text, "qux"), vec![]);
}
