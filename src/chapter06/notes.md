# Chapter 6: Enums and Pattern Matching

Enums allow you to define a type by enumerating its possible values. Pattern matching lets you run code based on the value of an enum.

## Defining Enums
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

## Using Enums
```rust
let m = Message::Write(String::from("hello"));
```

## The Option Enum
- Used for values that can be something or nothing.
```rust
let some_number = Some(5);
let absent_number: Option<i32> = None;
```

## Pattern Matching with match
```rust
let coin = Coin::Penny;
match coin {
    Coin::Penny => println!("1 cent"),
    Coin::Nickel => println!("5 cents"),
    Coin::Dime => println!("10 cents"),
    Coin::Quarter => println!("25 cents"),
}
```

## The if let Syntax
```rust
if let Some(x) = some_number {
    println!("{}", x);
}
```

---

# Quizzes
- Quiz 1: Define an enum for traffic lights (Red, Yellow, Green).
- Quiz 2: Write a function that takes a traffic light and returns how many seconds it stays on.
- Quiz 3: Write a function that takes an Option<i32> and returns the value or 0 if None.
