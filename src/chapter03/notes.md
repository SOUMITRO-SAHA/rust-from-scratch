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

## Loops in Rust

Rust provides three main types of loops:

### 1. `loop`

- An infinite loop unless you use `break`.
- Use for repeating code until you explicitly exit.

```rust
let mut count = 0;
loop {
    count += 1;
    if count == 3 {
        break;
    }
    println!("count = {}", count);
}
```

### 2. `while` loop

- Runs as long as a condition is true.

```rust
let mut n = 1;
while n <= 5 {
    println!("{}", n);
    n += 1;
}
```

### 3. `for` loop

- Iterate over a range or collection.
- Most idiomatic for iterating in Rust.

```rust
for n in 1..=5 {
    println!("{}", n);
}

let arr = [10, 20, 30];
for val in arr.iter() {
    println!("value: {}", val);
}
```

#### Best Practices

- Prefer `for` loops for collections and ranges.
- Use `loop` for indefinite repetition with explicit exit.
- Use `while` for condition-based repetition.

---

# Quizzes

- Quiz 1: Write a function that adds two numbers.
- Quiz 2: Write a function that returns the largest element in an array.
- Quiz 3: Write a function that prints numbers from 1 to 5 using a for loop.
