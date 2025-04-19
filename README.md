# Rust Learning Project

Welcome to your structured Rust learning project! This project is based on [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

## How to Use

1. **Read** the short Rustdoc summary in each chapter module (`lib.rs`).
2. **Attempt the quiz** by editing the code in `quiz.rs` for each chapter.
3. **Test your solution** using the provided `Makefile` commands for each chapter quiz:

   ```sh
   make run_quiz01   # Runs tests for Chapter 1
   make run_quiz02   # Runs tests for Chapter 2
   # ... and so on for each chapter
   ```

   This will automatically run the appropriate test using `cargo test --test quiz_chapterXX` for the given chapter.

4. **Before running a chapter or quiz test**, make sure to uncomment the corresponding module(s) in `src/lib.rs` (they are commented out by default to prevent build errors for unfinished chapters). For example, to work on Chapter 4, uncomment:

   ```rust
   pub mod chapter04 {
       pub mod quiz;
   }
   ```

   Then re-comment it if you want to skip that chapter later.

5. **Continue** to the next chapter when you pass the quiz.

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
