# Chapter 1: Getting Started

Welcome to Rust! In this chapter, you'll learn what Rust is, how to install it, and how to write and run your first program.

## What is Rust?
- A systems programming language focused on safety, speed, and concurrency.
- Guarantees memory safety without a garbage collector.

## Installing Rust
- Use [rustup](https://rustup.rs/) to install Rust and Cargo (the Rust package manager).
- Check installation:
  ```sh
  rustc --version
  cargo --version
  ```

## Hello, World!
- Every Rust program starts with a `main` function.
- Use `println!` macro to print text to the console.

```rust
fn main() {
    println!("Hello, world!");
}
```

## Running Your Program
- Use `cargo run` to compile and run your project.

---

# Quiz 1
Write a function that returns the string `"Hello, world!"`.

# Quiz 2
Write a function that prints `"Learning Rust!"` to the console.
