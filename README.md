# Rust Learning Project

Welcome to your structured Rust learning project! This project is based on [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

## How to Use

1. **Read** the short Rustdoc summary in each chapter module (`lib.rs`).
2. **Attempt the quiz** by editing the code in `quiz.rs` for each chapter.
3. **Test your solution** by running `cargo test --test quiz_chapterXX` (replace XX with the chapter number).
4. **Continue** to the next chapter when you pass the quiz.

## Project Structure

```
rust-learning/
│
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── chapter01/
│   │   ├── lib.rs      # Notes and examples for Chapter 1
│   │   └── quiz.rs     # Quiz and solution for Chapter 1
│   ├── chapter02/
│   │   ├── lib.rs      # Notes and examples for Chapter 2
│   │   └── quiz.rs     # Quiz and solution for Chapter 2
│   └── ... (more chapters)
└── tests/
    ├── quiz_chapter01.rs
    ├── quiz_chapter02.rs
    └── ... (more quizzes)
```

- All notes are in Rustdoc format for easy reading with `cargo doc`.
- Each quiz has an associated test file.

Happy learning! 🚀
