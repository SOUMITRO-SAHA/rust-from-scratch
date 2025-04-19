/// Quiz 1: Use Box to store an integer on the heap.
pub fn box_integer(x: i32) -> Box<i32> {
    Box::new(x)
}

/// Quiz 2: Implement Drop for a custom type and show when it runs.
pub struct Droppable(pub &'static str);
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

/// Quiz 3: Demonstrate shared ownership with Rc.
use std::rc::Rc;
pub fn rc_shared(val: i32) -> (Rc<i32>, Rc<i32>) {
    let a = Rc::new(val);
    let b = Rc::clone(&a);
    (a, b)
}

/// Quiz 4: Use RefCell for interior mutability.
use std::cell::RefCell;
pub fn refcell_mutate(x: i32, y: i32) -> i32 {
    let cell = RefCell::new(x);
    *cell.borrow_mut() = y;
    *cell.borrow()
}

/// Quiz 5: Combine Rc and RefCell for multiple owners with mutation.
pub fn rc_refcell(val: i32) -> (Rc<RefCell<i32>>, Rc<RefCell<i32>>) {
    let a = Rc::new(RefCell::new(val));
    let b = Rc::clone(&a);
    (a, b)
}

/// Quiz 6: Explain and demonstrate a memory leak with Rc/RefCell cycles.
/// (Written exercise: see notes.md for explanation.)
