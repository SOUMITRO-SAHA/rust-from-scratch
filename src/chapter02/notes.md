# Chapter 2: Programming a Guessing Game

This chapter introduces variables, mutability, data types, user input, and basic error handling in Rust.

## Variables and Mutability
- Variables are immutable by default.
- Use `mut` to make a variable mutable.

```rust
let x = 5; // immutable
let mut y = 10; // mutable
y = 15;
```

## Data Types
- Rust is statically typed. Types must be known at compile time.
- Common types: `i32`, `u32`, `f64`, `bool`, `char`, `String`.

```rust
let number: i32 = 42;
let name: String = String::from("Alice");
```

## Getting User Input
- Use `std::io` to read input from the user.

```rust
use std::io;
let mut guess = String::new();
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

## Error Handling
- Use `expect` to handle errors simply.

```rust
let num: i32 = guess.trim().parse().expect("Please type a number!");
```

---

# Quiz 1
Make a mutable variable, set it to 5, then change it to 10. Return the value.

# Quiz 2
Parse a string `"42"` into an integer and return it.

# Quiz 3
Write a function that takes a string and returns its length.
