use std::fs::read_to_string;
use std::path::Path;

#[macro_export]
macro_rules! len {
    ( $x:expr ) => {
        $x.len()
    };
}

pub fn read_lines(filename: &Path) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .map(|s| s.trim().to_owned())
        .collect()  // gather them together into a vector
}

pub fn pad_lines(lines: &Vec<String>) -> Vec<String> {
    let mut new_vec = lines.clone();
    for line in new_vec.iter_mut() {
        line.insert_str(0, ".");
        line.insert_str(len!(line), ".");
    }
    new_vec.insert(0, ".".repeat(new_vec[0].len()));
    new_vec.insert(len!(new_vec), ".".repeat(new_vec[0].len()));
    return new_vec;
}
