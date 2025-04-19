# Chapter 14: More About Cargo and Crates (Deep Dive)

This chapter covers advanced features of Cargo (Rust’s package manager and build tool) and crates (Rust libraries/packages). You’ll learn about workspaces, publishing, versioning, documentation, and more.

---

## 1. Cargo Workspaces
- A workspace is a set of packages that share the same Cargo.lock and output directory.
- Useful for organizing related libraries and binaries in one repo.

### Creating a Workspace
```toml
# Cargo.toml in the workspace root
[workspace]
members = [
    "my-lib",
    "my-app"
]
```
- Each member has its own `Cargo.toml`.

### Benefits
- Shared dependencies, easier dependency management, atomic commits across packages.

---

## 2. Publishing Crates
- To share your code, publish to [crates.io](https://crates.io/).
- Add metadata to `Cargo.toml`:
```toml
[package]
name = "my_crate"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2021"
description = "A cool crate"
license = "MIT"
repository = "https://github.com/your/repo"
```
- Run `cargo publish` (requires account and verification).

---

## 3. Crate Versioning and Dependencies
- Semantic versioning: `1.2.3` means `major.minor.patch`.
- In `Cargo.toml`, dependencies can use version ranges:
```toml
rand = "0.8"
serde = ">=1.0, <2.0"
```
- Use `cargo update` to update dependencies.

---

## 4. Documentation and Doc Tests
- Write documentation comments with `///` and `//!`.
- Generate docs with `cargo doc --open`.
- Code blocks in docs are automatically tested!
```rust
/// Adds two numbers.
///
/// # Examples
/// ```
/// assert_eq!(my_crate::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 5. Build Profiles and Features
- Profiles: `[profile.dev]`, `[profile.release]` in `Cargo.toml`.
- Features: opt-in compilation options, e.g.:
```toml
[features]
default = ["foo"]
foo = []
bar = []
```
- Use `--features` and `--no-default-features` on the command line.

---

## 6. Custom Build Scripts
- Use `build.rs` for custom build steps (e.g., code generation).
- Communicate with Cargo via environment variables.

---

## 7. Cargo Commands Reference
- `cargo check`, `cargo build`, `cargo run`, `cargo test`, `cargo clean`, `cargo update`, `cargo doc`, `cargo publish`, `cargo install`, `cargo tree`, `cargo bench`, etc.

---

# Quizzes
- Quiz 1: Create a workspace with two members and describe the benefits.
- Quiz 2: Write a documented function with a doc test.
- Quiz 3: Specify a dependency with a version range and explain semantic versioning.
- Quiz 4: Add a custom feature to a crate and explain how to enable it.
- Quiz 5: Write a custom build script that prints a message at build time.
