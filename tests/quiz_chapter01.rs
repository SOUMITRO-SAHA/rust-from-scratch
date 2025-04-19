// Quiz test for Chapter 1
use rust::chapter01::quiz;

#[test]
fn test_hello_world() {
    assert_eq!(quiz::hello_world(), "Hello, world!");
}

#[test]
fn test_print_learning_rust() {
    use std::io::{self, Write};
    let mut buf = Vec::new();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let orig = std::mem::replace(&mut handle, io::BufWriter::new(&mut buf));
    quiz::print_learning_rust();
    std::mem::replace(&mut handle, orig);
    let output = String::from_utf8(buf).expect("Not UTF-8");
    assert!(output.contains("Learning Rust!"));
}
