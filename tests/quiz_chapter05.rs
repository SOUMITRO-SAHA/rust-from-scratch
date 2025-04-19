use rust::chapter05::quiz::*;

#[test]
fn test_book_struct() {
    let book = Book { title: String::from("Rust Book"), pages: 350 };
    assert_eq!(book.title, "Rust Book");
    assert_eq!(book.pages, 350);
}

#[test]
fn test_is_long() {
    let short_book = Book { title: String::from("Short"), pages: 100 };
    let long_book = Book { title: String::from("Long"), pages: 400 };
    assert!(!short_book.is_long());
    assert!(long_book.is_long());
}

#[test]
fn test_point_struct() {
    let p = Point(3, 4);
    assert_eq!(p.0, 3);
    assert_eq!(p.1, 4);
}
