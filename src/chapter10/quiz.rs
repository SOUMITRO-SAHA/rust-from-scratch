/// Quiz 1: Write a generic function that returns the largest item in a slice.
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // TODO: Return the largest item
    &list[0]
}

/// Quiz 2: Define a trait and implement it for a struct.
pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Animal {
    pub name: String,
}

impl Describable for Animal {
    fn describe(&self) -> String {
        // TODO: Implement
        String::new()
    }
}

/// Quiz 3: Write a function that returns the longer of two string slices using lifetimes.
pub fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    // TODO: Return the longer slice
    x
}
