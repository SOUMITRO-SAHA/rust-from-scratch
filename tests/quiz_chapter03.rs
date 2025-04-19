use rust::chapter03::quiz;

#[test]
fn test_add() {
    assert_eq!(quiz::add(2, 3), 5);
    assert_eq!(quiz::add(-1, 1), 0);
}

#[test]
fn test_largest() {
    assert_eq!(quiz::largest(&[1, 2, 3, 4, 5]), 5);
    assert_eq!(quiz::largest(&[-10, -20, -3]), -3);
}

#[test]
fn test_print_1_to_5_for() {
    quiz::print_1_to_5_for();
}

#[test]
fn test_print_1_to_5_while() {
    quiz::print_1_to_5_while();
}

#[test]
fn test_print_1_to_5_loop() {
    quiz::print_1_to_5_loop();
}
