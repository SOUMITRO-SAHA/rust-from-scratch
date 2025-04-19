# Chapter 4: Understanding Ownership

Ownership is Rust’s most unique feature and enables memory safety without a garbage collector.

## What is Ownership?
- Each value in Rust has a variable that’s its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value is dropped.

## Move Semantics
- Assigning a value to another variable moves ownership.

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is now invalid
```

## Clone
- Use `.clone()` to deeply copy heap data.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

## Borrowing and References
- Use `&` to borrow a value without taking ownership.
- Mutable references: `&mut`.

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}

let mut s = String::from("hello");
let len = calculate_length(&s);
```

## Slices
- Slices let you reference a contiguous sequence of elements.

```rust
let s = String::from("hello world");
let hello = &s[0..5];
```

---

# Quizzes
- Quiz 1: Write a function that takes ownership of a String and returns its length.
- Quiz 2: Write a function that borrows a String and returns its length.
- Quiz 3: Write a function that returns the first word of a string slice.
