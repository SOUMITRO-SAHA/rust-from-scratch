# Chapter 16: Fearless Concurrency (Deep Dive)

Rust’s concurrency model enables safe and efficient parallel programming. This chapter covers threads, message passing, shared state, and synchronization.

---

## 1. Threads
- Spawn new threads with `std::thread::spawn`.
- Use `join()` to wait for threads to finish.
```rust
use std::thread;
let handle = thread::spawn(|| {
    println!("Hello from a thread!");
});
handle.join().unwrap();
```

---

## 2. Message Passing
- Channels for communication between threads: `std::sync::mpsc`.
```rust
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
tx.send(42).unwrap();
println!("Received: {}", rx.recv().unwrap());
```

---

## 3. Shared-State Concurrency
- Use `Arc<T>` for shared ownership across threads.
- Use `Mutex<T>` for safe, mutable access.
```rust
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
let c1 = Arc::clone(&counter);
let handle = std::thread::spawn(move || {
    let mut num = c1.lock().unwrap();
    *num += 1;
});
handle.join().unwrap();
```

---

## 4. Deadlocks and Poisoning
- Deadlocks: when threads wait forever for each other.
- Poisoning: when a thread panics while holding a lock.

---

## 5. Send and Sync Traits
- Types that are safe to transfer or reference across threads.

---

# Quizzes
- Quiz 1: Spawn a thread and print a message.
- Quiz 2: Use a channel to send and receive a value between threads.
- Quiz 3: Use Arc and Mutex to share and mutate data across threads.
- Quiz 4: Demonstrate a deadlock and explain how to avoid it.
- Quiz 5: Explain Send and Sync traits and check if a type implements them.
