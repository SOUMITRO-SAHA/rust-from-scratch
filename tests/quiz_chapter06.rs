use rust::chapter06::quiz::*;

#[test]
fn test_traffic_light_enum() {
    let _r = TrafficLight::Red;
    let _y = TrafficLight::Yellow;
    let _g = TrafficLight::Green;
}

#[test]
fn test_light_duration() {
    assert_eq!(light_duration(TrafficLight::Red), 60);
    assert_eq!(light_duration(TrafficLight::Yellow), 5);
    assert_eq!(light_duration(TrafficLight::Green), 30);
}

#[test]
fn test_value_or_zero() {
    assert_eq!(value_or_zero(Some(10)), 10);
    assert_eq!(value_or_zero(None), 0);
}
