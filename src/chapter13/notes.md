# Chapter 13: Functional Language Features: Iterators and Closures

Rust supports many functional programming concepts, such as closures, iterators, and higher-order functions. This chapter explores these features in depth, with practical code examples and explanations.

---

## Closures
Closures are anonymous functions you can save in a variable or pass as arguments to other functions. They capture variables from their environment.

### Syntax and Usage
```rust
let add_one = |x: i32| x + 1;
assert_eq!(add_one(5), 6);
```
Closures can infer parameter and return types from context, but you can annotate them explicitly:
```rust
let multiply = |a: i32, b: i32| -> i32 { a * b };
```

### Capturing Environment
Closures can capture variables by reference, mutable reference, or by value (move):
```rust
let mut num = 5;
let mut add_num = |x: i32| num += x;
add_num(3);
assert_eq!(num, 8);
```

---

## Iterators
Iterators allow you to process a sequence of elements. They are lazy, meaning computation happens only when needed.

### Creating Iterators
```rust
let v = vec![1, 2, 3];
let mut iter = v.iter();
assert_eq!(iter.next(), Some(&1));
```

### Consuming Adaptors
- `sum`, `collect`, `count`, `last`, etc.
```rust
let v = vec![1, 2, 3];
let total: i32 = v.iter().sum();
assert_eq!(total, 6);
```

### Iterator Adaptors (Transformers)
- `map`, `filter`, `take`, `skip`, `enumerate`, etc.
```rust
let v = vec![1, 2, 3, 4, 5];
let evens: Vec<_> = v.into_iter().filter(|x| x % 2 == 0).collect();
assert_eq!(evens, vec![2, 4]);
```

### Chaining Iterator Methods
```rust
let words = vec!["hello", "world", "rust"];
let result: Vec<_> = words.iter().map(|w| w.len()).filter(|&len| len > 4).collect();
assert_eq!(result, vec![5]);
```

---

## Implementing Iterator for Your Types
You can implement the `Iterator` trait for your own types:
```rust
struct Counter { count: usize }

impl Iterator for Counter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 { Some(self.count) } else { None }
    }
}
```

---

## Common Pitfalls and Best Practices
- Prefer iterators over indexed loops for safety and clarity.
- Use closures for short, throwaway functions.
- Remember that iterator adaptors are lazy; use a consuming adaptor to trigger computation.
- Use `.cloned()` or `.copied()` if you need owned values from an iterator over references.

---

# Quizzes
- Quiz 1: Write a closure that multiplies two numbers.
- Quiz 2: Use an iterator to sum even numbers in a vector.
- Quiz 3: Implement a custom iterator that yields the squares of numbers from 1 to 5.
- Quiz 4: Use `map` and `filter` to process a list of strings, keeping only those with length > 3 and converting them to uppercase.
