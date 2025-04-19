use rust::chapter13::quiz::*;

#[test]
fn test_multiply_closure() {
    let mul = multiply_closure();
    assert_eq!(mul(3, 4), 12);
    assert_eq!(mul(0, 100), 0);
}

#[test]
fn test_sum_evens() {
    assert_eq!(sum_evens(vec![1, 2, 3, 4, 5, 6]), 12);
    assert_eq!(sum_evens(vec![1, 3, 5]), 0);
}

#[test]
fn test_squares_iterator() {
    let squares: Vec<_> = Squares::new().collect();
    assert_eq!(squares, vec![1, 4, 9, 16, 25]);
}

#[test]
fn test_process_strings() {
    let input = vec!["hi", "rust", "language", "go"];
    let output = process_strings(input);
    assert_eq!(output, vec!["RUST", "LANGUAGE"]);
}

#[test]
fn test_join_strings() {
    let words = vec!["a", "b", "c"];
    assert_eq!(join_strings(words, ","), "a,b,c");
    let empty: Vec<&str> = vec![];
    assert_eq!(join_strings(empty, ","), "");
}

#[test]
fn test_even_indices() {
    assert_eq!(even_indices(vec![1,2,3,4,5,6]), vec![1,3,5]);
    assert_eq!(even_indices(vec![1,3,5]), vec![]);
}

#[test]
fn test_first_consecutive_pair() {
    assert_eq!(first_consecutive_pair(vec![1,2,2,3]), Some((2,2)));
    assert_eq!(first_consecutive_pair(vec![1,2,3,4]), None);
    assert_eq!(first_consecutive_pair(vec![7,7]), Some((7,7)));
}
