# Chapter 20: Macros (Deep Dive)

Macros are a powerful metaprogramming feature in Rust, allowing you to write code that writes code. This chapter covers declarative macros, procedural macros, and macro hygiene.

---

## 1. Declarative Macros (macro_rules!)
- Define patterns and code generation rules.
```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}
```

---

## 2. Procedural Macros
- Custom derives, attribute-like, and function-like macros.
- Use crates like `syn`, `quote`, and `proc-macro`.

---

## 3. Macro Hygiene
- Prevents name conflicts and accidental variable capture.

---

## 4. When to Use Macros
- Reduce boilerplate, implement DSLs, or for code generation.

---

# Quizzes
- Quiz 1: Write a basic macro_rules! macro.
- Quiz 2: Use a macro to generate repetitive code.
- Quiz 3: Explain macro hygiene with an example.
- Quiz 4: Describe the difference between declarative and procedural macros.
- Quiz 5: Implement a custom derive procedural macro (outline only).
