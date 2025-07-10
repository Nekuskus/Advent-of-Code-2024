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
