/// Quiz 1: Spawn a thread and print a message.
pub fn spawn_thread() {
    std::thread::spawn(|| println!("Hello from a thread!"))
        .join()
        .unwrap();
}

/// Quiz 2: Use a channel to send and receive a value between threads.
pub fn channel_send_recv(val: i32) -> i32 {
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || tx.send(val).unwrap());
    rx.recv().unwrap()
}

/// Quiz 3: Use Arc and Mutex to share and mutate data across threads.
pub fn arc_mutex_increment() -> i32 {
    use std::sync::{Arc, Mutex};
    let counter = Arc::new(Mutex::new(0));
    let c1 = Arc::clone(&counter);
    let handle = std::thread::spawn(move || {
        let mut num = c1.lock().unwrap();
        *num += 1;
    });
    handle.join().unwrap();
    *counter.lock().unwrap()
}

/// Quiz 4: Demonstrate a deadlock and explain how to avoid it.
/// (Written exercise: see notes.md for explanation.)

/// Quiz 5: Explain Send and Sync traits and check if a type implements them.
pub fn is_send_sync<T: Send + Sync>() {}
