/// Quiz 1: Write a closure that multiplies two numbers.
pub fn multiply_closure() -> impl Fn(i32, i32) -> i32 {
    |a, b| a * b
}

/// Quiz 2: Use an iterator to sum even numbers in a vector.
pub fn sum_evens(v: Vec<i32>) -> i32 {
    v.into_iter().filter(|x| x % 2 == 0).sum()
}

/// Quiz 3: Implement a custom iterator that yields the squares of numbers from 1 to 5.
pub struct Squares {
    curr: usize,
}

impl Squares {
    pub fn new() -> Self {
        Squares { curr: 0 }
    }
}

impl Iterator for Squares {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr += 1;
        if self.curr <= 5 {
            Some(self.curr * self.curr)
        } else {
            None
        }
    }
}

/// Quiz 4: Use map and filter to process a list of strings, keeping only those with length > 3 and converting them to uppercase.
pub fn process_strings(strings: Vec<&str>) -> Vec<String> {
    strings.into_iter()
        .filter(|s| s.len() > 3)
        .map(|s| s.to_uppercase())
        .collect()
}

/// Quiz 5: Use fold to concatenate a list of strings with a separator.
pub fn join_strings(strings: Vec<&str>, sep: &str) -> String {
    strings.into_iter().fold(String::new(), |mut acc, s| {
        if !acc.is_empty() { acc.push_str(sep); }
        acc.push_str(s);
        acc
    })
}

/// Quiz 6: Use enumerate to return indices of even numbers in a vector.
pub fn even_indices(v: Vec<i32>) -> Vec<usize> {
    v.into_iter().enumerate().filter_map(|(i, x)| if x % 2 == 0 { Some(i) } else { None }).collect()
}

/// Quiz 7: Use peekable to return the first pair of consecutive equal elements in a vector.
pub fn first_consecutive_pair(v: Vec<i32>) -> Option<(i32, i32)> {
    let mut iter = v.into_iter().peekable();
    while let Some(curr) = iter.next() {
        if let Some(&next) = iter.peek() {
            if curr == next {
                return Some((curr, next));
            }
        }
    }
    None
}
