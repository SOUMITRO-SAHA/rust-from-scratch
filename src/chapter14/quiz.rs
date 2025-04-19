/// Quiz 1: Create a workspace with two members and describe the benefits.
/// (This is a written exercise; see notes.md for instructions.)

/// Quiz 2: Write a documented function with a doc test.
/// Adds two numbers.
///
/// # Examples
/// ```
/// use rust::chapter14::quiz::add;
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Quiz 3: Specify a dependency with a version range and explain semantic versioning.
/// (This is a written exercise; see notes.md for instructions.)

/// Quiz 4: Add a custom feature to a crate and explain how to enable it.
#[cfg(feature = "fancy")]
pub fn feature_demo() -> &'static str {
    "Fancy feature enabled!"
}
#[cfg(not(feature = "fancy"))]
pub fn feature_demo() -> &'static str {
    "Fancy feature not enabled."
}

/// Quiz 5: Write a custom build script that prints a message at build time.
/// (This is a written/code exercise; see notes.md for instructions.)
