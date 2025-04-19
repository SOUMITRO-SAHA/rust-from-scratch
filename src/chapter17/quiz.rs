/// Quiz 1: Define a trait and implement it for a struct.
pub trait Greet {
    fn greet(&self) -> String;
}
pub struct Person;
impl Greet for Person {
    fn greet(&self) -> String {
        "Hello!".to_string()
    }
}

/// Quiz 2: Use a trait object for dynamic dispatch.
pub fn dynamic_greet(obj: &dyn Greet) -> String {
    obj.greet()
}

/// Quiz 3: Implement the state pattern using traits and structs.
pub trait State {
    fn next(self: Box<Self>) -> Box<dyn State>;
    fn status(&self) -> &'static str;
}
pub struct Start;
pub struct End;
impl State for Start {
    fn next(self: Box<Self>) -> Box<dyn State> { Box::new(End) }
    fn status(&self) -> &'static str { "Start" }
}
impl State for End {
    fn next(self: Box<Self>) -> Box<dyn State> { self }
    fn status(&self) -> &'static str { "End" }
}

/// Quiz 4: Provide a default implementation for a trait method.
pub trait DefaultHello {
    fn hello(&self) -> String { "Hello, default!".to_string() }
}

/// Quiz 5: Use a supertrait to require multiple behaviors.
pub trait A { fn a(&self) -> i32; }
pub trait B: A { fn b(&self) -> i32; }
