# Chapter 9: Error Handling

Rust’s approach to error handling is designed to help you write robust programs.

## Unrecoverable Errors with panic!
- Use `panic!` when a program encounters an unrecoverable error.
```rust
panic!("Something went wrong!");
```

## Recoverable Errors with Result
- The `Result` enum is used for functions that can succeed or fail.
```rust
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(x / y)
    }
}
```
- Use `match`, `unwrap`, or `expect` to handle results.

## The ? Operator
- Propagates errors to the calling function.
```rust
fn read_file() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("foo.txt")?;
    Ok(content)
}
```

---

# Quizzes
- Quiz 1: Write a function that divides two numbers and returns a Result.
- Quiz 2: Write a function that returns an error if a string is empty.
- Quiz 3: Use the `?` operator to read a file and return its contents.
