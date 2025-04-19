# Chapter 12: An I/O Project - Building a Command Line Program

This chapter guides you through building a command-line program in Rust, focusing on reading arguments, files, and writing tests.

## Accepting Command Line Arguments
- Use `std::env::args` to get arguments.
```rust
let args: Vec<String> = std::env::args().collect();
```

## Reading a File
- Use `std::fs::read_to_string` to read file contents.

## Writing Functions and Tests
- Split logic into functions for easier testing.
- Write tests for your functions.

## Using Closures and the Environment
- Use closures for flexible searching.
- Use `env::var` to read environment variables.

---

# Quizzes
- Quiz 1: Write a function that returns the number of command line arguments.
- Quiz 2: Write a function that reads a file and returns its contents.
- Quiz 3: Write a function that searches for a string in a file's contents and returns the line numbers where it appears.
