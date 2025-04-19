/// Quiz 1: Write a function that returns the number of command line arguments.
pub fn arg_count() -> usize {
    // TODO: Return number of command line arguments
    0
}

/// Quiz 2: Write a function that reads a file and returns its contents.
pub fn read_file(path: &str) -> Result<String, std::io::Error> {
    // TODO: Use std::fs::read_to_string
    Err(std::io::Error::new(std::io::ErrorKind::Other, "not implemented"))
}

/// Quiz 3: Write a function that searches for a string in a file's contents and returns the line numbers where it appears.
pub fn search_lines(contents: &str, query: &str) -> Vec<usize> {
    // TODO: Return line numbers (1-based) where query appears
    vec![]
}
