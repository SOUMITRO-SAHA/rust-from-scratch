use rust::chapter14::quiz::*;

#[test]
fn test_add_doc_test() {
    assert_eq!(add(2, 2), 4);
    assert_eq!(add(-1, 1), 0);
}

#[test]
fn test_feature_demo() {
    // This test will pass regardless of feature flag, but output will differ
    let msg = feature_demo();
    assert!(msg == "Fancy feature enabled!" || msg == "Fancy feature not enabled.");
}
