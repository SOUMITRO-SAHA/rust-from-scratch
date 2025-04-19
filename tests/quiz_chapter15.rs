use rust::chapter15::quiz::*;

#[test]
fn test_box_integer() {
    let b = box_integer(42);
    assert_eq!(*b, 42);
}

#[test]
fn test_drop_runs() {
    // This test is for manual observation; Drop prints a message.
    let _d = Droppable("test");
}

#[test]
fn test_rc_shared() {
    let (a, b) = rc_shared(10);
    assert_eq!(*a, 10);
    assert_eq!(*b, 10);
}

#[test]
fn test_refcell_mutate() {
    assert_eq!(refcell_mutate(1, 9), 9);
}

#[test]
fn test_rc_refcell() {
    let (a, b) = rc_refcell(5);
    *a.borrow_mut() += 1;
    assert_eq!(*b.borrow(), 6);
}
