# Chapter 11: Writing Automated Tests

Testing is a core part of Rust development. Rust has first-class support for unit and integration tests.

## Writing Tests
- Use `#[test]` above functions to mark them as tests.
```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
```

## Checking Results with assert!
- `assert!`, `assert_eq!`, and `assert_ne!` are used for checks.

## Custom Failure Messages
- Add a message as the last argument to `assert!` macros.

## Testing Panics
- Use `#[should_panic]` to check for panics.

## Testing Expected Errors
- Use `Result<T, E>` as the return type of a test function.

## Test Organization
- Unit tests go in the same file, in a `mod tests` module.
- Integration tests go in the `tests/` directory.

---

# Quizzes
- Quiz 1: Write a function and a test that asserts it returns the correct value.
- Quiz 2: Write a test that checks for a panic.
- Quiz 3: Write an integration test in the `tests/` directory for a public function.
