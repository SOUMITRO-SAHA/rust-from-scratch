/// Quiz 1: Create a module named `math` with a function `add` that adds two numbers.
pub mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        // TODO: Implement addition
        0
    }
}

/// Quiz 2: Use `use` to bring `add` into scope and call it from main.
pub fn call_add(x: i32, y: i32) -> i32 {
    // TODO: Use `use` and call math::add
    0
}

/// Quiz 3: Create a nested module structure and call a function from the innermost module.
pub mod outer {
    pub mod inner {
        pub fn hello() -> &'static str {
            // TODO: Return a greeting
            ""
        }
    }
}

pub fn call_hello() -> &'static str {
    // TODO: Call outer::inner::hello
    ""
}
