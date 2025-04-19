# Chapter 15: Smart Pointers (Deep Dive)

Smart pointers are data structures that act like pointers but have additional metadata and capabilities. Rust’s standard library includes several smart pointers, each with unique features and use cases.

---

## 1. Box<T>
- Heap allocation for single values.
- Use when you need to own data on the heap or for recursive types.
```rust
let b = Box::new(5);
println!("b = {}", b);
```

---

## 2. Deref and DerefMut Traits
- Allow custom types to behave like references.
- Used by `Box`, `Rc`, etc.
- Enables deref coercion (automatic conversion to reference).

---

## 3. Drop Trait
- Runs code when a value goes out of scope (destructor).
```rust
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping {}", self.data);
    }
}
```

---

## 4. Rc<T>
- Reference counting for shared ownership.
- Use when multiple parts of your program need to read the same data.
```rust
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a);
```

---

## 5. RefCell<T> and Interior Mutability
- Allows mutation through an immutable reference at runtime.
- Checked at runtime (not compile time).
- Use for single-threaded scenarios needing "mutable borrow checking" at runtime.
```rust
use std::cell::RefCell;
let x = RefCell::new(42);
*x.borrow_mut() = 43;
```

---

## 6. Rc<RefCell<T>>
- Combine for multiple owners with interior mutability.

---

## 7. Weak<T>
- Non-owning reference to data managed by `Rc<T>`; prevents reference cycles.

---

## 8. Cycles and Memory Leaks
- Rust prevents most memory leaks, but reference cycles with `Rc`/`RefCell` can leak.

---

# Quizzes
- Quiz 1: Use Box to store an integer on the heap.
- Quiz 2: Implement Drop for a custom type and show when it runs.
- Quiz 3: Demonstrate shared ownership with Rc.
- Quiz 4: Use RefCell for interior mutability.
- Quiz 5: Combine Rc and RefCell for multiple owners with mutation.
- Quiz 6: Explain and demonstrate a memory leak with Rc/RefCell cycles.
