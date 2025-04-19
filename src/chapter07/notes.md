# Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

As your Rust projects grow, organizing code with packages, crates, and modules becomes crucial.

## Packages and Crates
- **Package:** A bundle of one or more crates. Has a Cargo.toml.
- **Crate:** A binary or library. The smallest amount of code that the Rust compiler considers at a time.

## Defining Modules
- Use `mod` to define modules.
- Files and folders can organize modules.

```rust
mod garden {
    pub mod vegetables {
        pub fn carrot() {
            println!("I'm a carrot!");
        }
    }
}

fn main() {
    garden::vegetables::carrot();
}
```

## Paths for Referring to an Item
- Absolute paths start from the crate root: `crate::garden::vegetables::carrot`.
- Relative paths start from the current module: `self::some_fn` or `super::parent_fn`.

## Bringing Paths into Scope with `use`
```rust
use crate::garden::vegetables::carrot;
carrot();
```

---

# Quizzes
- Quiz 1: Create a module named `math` with a function `add` that adds two numbers.
- Quiz 2: Use `use` to bring `add` into scope and call it from main.
- Quiz 3: Create a nested module structure and call a function from the innermost module.
