use rust::chapter11::quiz::greet;

#[test]
fn test_integration_greet() {
    assert_eq!(greet("Rustacean"), "Hello, Rustacean!");
}
