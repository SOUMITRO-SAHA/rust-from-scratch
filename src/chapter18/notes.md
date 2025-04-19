# Chapter 18: Patterns and Matching (Deep Dive)

Patterns are used throughout Rust for destructuring, matching, and control flow.

---

## 1. Pattern Syntax
- Match literals, variables, wildcards, ranges, enums, structs, tuples, arrays, references, and more.

---

## 2. match Expressions
- Powerful control flow with exhaustive checking.
```rust
match value {
    1 => println!("one"),
    2..=5 => println!("two to five"),
    _ => println!("something else"),
}
```

---

## 3. if let, while let, and let-else
- Syntactic sugar for matching specific patterns.

---

## 4. Destructuring Structs, Enums, and Tuples
- Extract fields and values easily.

---

## 5. Ignoring Values and Nested Patterns
- Use `_`, `..`, and nested patterns for flexibility.

---

## 6. Guards and Bindings
- Add extra conditions to patterns with `if` guards.
- Bind matched values with `@`.

---

# Quizzes
- Quiz 1: Match on an enum and return a value.
- Quiz 2: Use if let and while let for control flow.
- Quiz 3: Destructure a struct in a match.
- Quiz 4: Use guards and bindings in a match.
- Quiz 5: Write a pattern that matches only tuples with the first element as 0.
