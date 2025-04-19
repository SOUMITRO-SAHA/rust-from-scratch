# Chapter 8: Common Collections

Rust’s standard library includes several useful collections for storing data.

## Vectors
- Growable arrays. Store values of the same type.
```rust
let mut v: Vec<i32> = Vec::new();
v.push(1);
v.push(2);
let v2 = vec![1, 2, 3];
```
- Access elements with `[]` or `.get()`.

## Strings
- UTF-8 encoded, growable text.
```rust
let mut s = String::from("hello");
s.push_str(" world");
```
- Indexing is not allowed; use slicing.

## Hash Maps
- Store key-value pairs. Keys and values can be different types.
```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
```
- Access with `.get()`; iterate with a for loop.

---

# Quizzes
- Quiz 1: Create a vector of integers and return the sum.
- Quiz 2: Append a string to another string and return the result.
- Quiz 3: Insert and retrieve a value from a hash map.
