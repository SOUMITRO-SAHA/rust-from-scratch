use rust::chapter17::quiz::*;

#[test]
fn test_greet_trait() {
    let p = Person;
    assert_eq!(p.greet(), "Hello!");
}

#[test]
fn test_dynamic_greet() {
    let p = Person;
    assert_eq!(dynamic_greet(&p), "Hello!");
}

#[test]
fn test_state_pattern() {
    let mut state: Box<dyn State> = Box::new(Start);
    assert_eq!(state.status(), "Start");
    state = state.next();
    assert_eq!(state.status(), "End");
}

#[test]
fn test_default_trait_impl() {
    struct Dummy;
    impl DefaultHello for Dummy {}
    let d = Dummy;
    assert_eq!(d.hello(), "Hello, default!");
}

#[test]
fn test_supertrait() {
    struct ABImpl;
    impl A for ABImpl { fn a(&self) -> i32 { 1 } }
    impl B for ABImpl { fn b(&self) -> i32 { 2 } }
    let ab = ABImpl;
    assert_eq!(ab.a(), 1);
    assert_eq!(ab.b(), 2);
}
