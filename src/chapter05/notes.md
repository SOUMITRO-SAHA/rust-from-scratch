# Chapter 5: Using Structs to Structure Related Data

Structs let you create custom types that group related data together.

## Defining and Instantiating Structs
```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    username: String::from("someone"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
    active: true,
};
```

## Field Init Shorthand & Struct Update Syntax
```rust
let username = String::from("another");
let user2 = User { username, ..user1 };
```

## Tuple Structs & Unit-Like Structs
```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

## Ownership of Struct Data
- Each field in a struct has its own ownership rules.

## Methods
- Define methods using `impl` blocks.
```rust
impl User {
    fn email_domain(&self) -> &str {
        self.email.split('@').nth(1).unwrap_or("")
    }
}
```

---

# Quizzes
- Quiz 1: Define a `Book` struct with `title` and `pages` fields.
- Quiz 2: Implement a method on `Book` that returns `true` if pages > 300.
- Quiz 3: Create a tuple struct for a 2D point.
