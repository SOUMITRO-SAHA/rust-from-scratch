# Chapter 17: Object-Oriented Programming Features (Deep Dive)

Rust supports many object-oriented programming (OOP) concepts, including encapsulation, inheritance (via traits), and polymorphism.

---

## 1. Encapsulation with Structs and Methods
- Use `pub`/private fields and methods to control visibility.

---

## 2. Inheritance via Traits
- Traits define shared behavior; structs implement traits.
- Trait bounds enable polymorphism.

---

## 3. Dynamic Dispatch and Trait Objects
- Use `dyn Trait` for runtime polymorphism.
- Example:
```rust
trait Draw {
    fn draw(&self);
}
impl Draw for Button {
    fn draw(&self) { println!("Drawing a button"); }
}
let b = Button {};
let obj: &dyn Draw = &b;
obj.draw();
```

---

## 4. State Pattern
- Use trait objects to represent state transitions in objects.

---

## 5. Default Implementations and Supertraits
- Traits can provide default method implementations.
- Supertraits: traits that require other traits.

---

# Quizzes
- Quiz 1: Define a trait and implement it for a struct.
- Quiz 2: Use a trait object for dynamic dispatch.
- Quiz 3: Implement the state pattern using traits and structs.
- Quiz 4: Provide a default implementation for a trait method.
- Quiz 5: Use a supertrait to require multiple behaviors.
