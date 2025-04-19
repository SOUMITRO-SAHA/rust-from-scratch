/// Quiz 1: Write a function that divides two numbers and returns a Result.
pub fn safe_divide(x: i32, y: i32) -> Result<i32, String> {
    // TODO: Return Ok(x / y) or Err("Cannot divide by zero")
    Err(String::from("not implemented"))
}

/// Quiz 2: Write a function that returns an error if a string is empty.
pub fn check_not_empty(s: &str) -> Result<&str, String> {
    // TODO: Return Ok(s) or Err("Empty string")
    Err(String::from("not implemented"))
}

/// Quiz 3: Use the `?` operator to read a file and return its contents.
pub fn read_file_quiz(path: &str) -> Result<String, std::io::Error> {
    // TODO: Use std::fs::read_to_string and ?
    Err(std::io::Error::new(std::io::ErrorKind::Other, "not implemented"))
}
