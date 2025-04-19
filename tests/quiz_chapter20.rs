// Macro tests must be run in integration tests or with #[macro_use] in main crate.
// Here we check macro presence and usage.
#[macro_use]
extern crate rust;

#[test]
fn test_hello_macro() {
    hello_macro!();
}

#[test]
fn test_make_functions() {
    make_functions!(foo, bar);
    foo();
    bar();
}
