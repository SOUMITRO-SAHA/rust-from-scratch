/// Quiz 1: Define a trait with an associated type and implement it.
pub trait MyIter {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
pub struct Counter { count: usize }
impl MyIter for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count <= 3 { Some(self.count) } else { None }
    }
}

/// Quiz 2: Overload an operator for a struct.
use std::ops::Add;
pub struct Point { pub x: i32, pub y: i32 }
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

/// Quiz 3: Use fully qualified syntax to call a trait method.
pub trait Foo { fn foo(&self) -> &'static str; }
pub struct Bar;
impl Foo for Bar { fn foo(&self) -> &'static str { "foo" } }
impl Bar { pub fn foo(&self) -> &'static str { "bar" } }
pub fn call_trait_method(b: &Bar) -> &'static str {
    <Bar as Foo>::foo(b)
}

/// Quiz 4: Implement a supertrait and show usage.
pub trait A { fn a(&self) -> i32; }
pub trait B: A { fn b(&self) -> i32; }
pub struct AB;
impl A for AB { fn a(&self) -> i32 { 1 } }
impl B for AB { fn b(&self) -> i32 { 2 } }

/// Quiz 5: Use the newtype pattern to implement a trait for an external type.
pub struct Wrapper(pub Vec<String>);
use std::fmt;
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
