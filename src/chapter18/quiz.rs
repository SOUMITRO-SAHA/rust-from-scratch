/// Quiz 1: Match on an enum and return a value.
pub enum MyEnum { A, B(i32), C(String) }
pub fn match_enum(e: MyEnum) -> String {
    match e {
        MyEnum::A => "A".to_string(),
        MyEnum::B(x) => format!("B({})", x),
        MyEnum::C(s) => format!("C({})", s),
    }
}

/// Quiz 2: Use if let and while let for control flow.
pub fn if_while_let(v: Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    let mut iter = v.into_iter();
    while let Some(Some(x)) = iter.next() {
        sum += x;
    }
    sum
}

/// Quiz 3: Destructure a struct in a match.
pub struct Point { pub x: i32, pub y: i32 }
pub fn match_struct(p: Point) -> i32 {
    match p {
        Point { x, y: 0 } => x,
        Point { x: 0, y } => y,
        Point { x, y } => x + y,
    }
}

/// Quiz 4: Use guards and bindings in a match.
pub fn match_guard(v: i32) -> &'static str {
    match v {
        x if x < 0 => "negative",
        x if x == 0 => "zero",
        x if x > 0 => "positive",
        _ => "unknown",
    }
}

/// Quiz 5: Write a pattern that matches only tuples with the first element as 0.
pub fn match_tuple(t: (i32, i32)) -> bool {
    match t {
        (0, _) => true,
        _ => false,
    }
}
