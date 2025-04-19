# Chapter 3: Common Programming Concepts

This chapter covers foundational concepts in Rust that are common to many programming languages, including variables, data types, functions, comments, control flow, and more.

## Variables and Mutability
- Variables are immutable by default. Use `mut` to make them mutable.

```rust
let x = 5;
let mut y = 10;
y = 15;
```

## Data Types
- Scalar types: integers, floating-point numbers, booleans, characters.
- Compound types: tuples, arrays.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (a, b, c) = tup;
let arr = [1, 2, 3, 4, 5];
```

## Functions
- Defined with `fn` keyword.
- Parameters must have types.

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

## Comments
- Single-line: `//`
- Multi-line/doc: `///`, `//!`

## Control Flow
- `if`, `else`, `else if` for conditional branching.
- `loop`, `while`, `for` for loops.

```rust
if x < 5 {
    println!("x is less than 5");
} else {
    println!("x is greater or equal to 5");
}

for n in 1..=5 {
    println!("{}", n);
}
```

---

# Quizzes
- Quiz 1: Write a function that adds two numbers.
- Quiz 2: Write a function that returns the largest element in an array.
- Quiz 3: Write a function that prints numbers from 1 to 5 using a for loop.
