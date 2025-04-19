# Chapter 10: Generic Types, Traits, and Lifetimes

Rust’s generics and traits enable flexible, reusable code.

## Generics
- Define functions, structs, enums, and methods for multiple types.
```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

## Traits
- Define shared behavior.
```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle { headline: String }
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}", self.headline)
    }
}
```

## Trait Bounds and `where` Clauses
- Restrict generic types with trait bounds.

## Lifetimes
- Prevent dangling references.
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

# Quizzes
- Quiz 1: Write a generic function that returns the largest item in a slice.
- Quiz 2: Define a trait and implement it for a struct.
- Quiz 3: Write a function that returns the longer of two string slices using lifetimes.
