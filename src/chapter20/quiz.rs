/// Quiz 1: Write a basic macro_rules! macro.
#[macro_export]
macro_rules! hello_macro {
    () => {
        println!("Hello, macro!");
    };
}

/// Quiz 2: Use a macro to generate repetitive code.
#[macro_export]
macro_rules! make_functions {
    ($($name:ident),*) => {
        $(fn $name() { println!("You called {:?}!", stringify!($name)); })*
    };
}

/// Quiz 3: Explain macro hygiene with an example.
/// (Written exercise: see notes.md for explanation.)

/// Quiz 4: Describe the difference between declarative and procedural macros.
/// (Written exercise: see notes.md for explanation.)

/// Quiz 5: Implement a custom derive procedural macro (outline only).
/// (Written/design exercise: see notes.md for outline.)
