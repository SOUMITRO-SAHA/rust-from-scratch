# Chapter 19: Advanced Traits (Deep Dive)

Traits are Rust’s way of defining shared behavior. This chapter explores advanced trait features, including associated types, default implementations, operator overloading, supertraits, and more.

---

## 1. Associated Types
- Define placeholder types in traits, implemented by concrete types.
```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

---

## 2. Default Generic Type Parameters and Operator Overloading
- Traits can have default type parameters.
- Implement traits for operator overloading (e.g., `Add`, `Sub`).
```rust
use std::ops::Add;
impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}
```

---

## 3. Fully Qualified Syntax
- Disambiguate methods with the same name from different traits.

---

## 4. Supertraits
- A trait that requires another trait.
```rust
trait Write: Display {
    fn write(&self);
}
```

---

## 5. Newtype Pattern
- Wrap types to implement external traits.

---

# Quizzes
- Quiz 1: Define a trait with an associated type and implement it.
- Quiz 2: Overload an operator for a struct.
- Quiz 3: Use fully qualified syntax to call a trait method.
- Quiz 4: Implement a supertrait and show usage.
- Quiz 5: Use the newtype pattern to implement a trait for an external type.
