use std::fs::read_to_string;
use std::path::Path;

pub fn read_lines(filename: &Path) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(|s| s.trim().to_owned())
        .collect()  // gather them together into a vector
}

pub fn pad_lines(lines: &Vec<String>) -> Vec<String> {
    let mut new_vec = lines.clone();
    for line in new_vec.iter_mut() {
        line.insert_str(0, ".");
        line.insert_str(line.len(), ".");
    }
    new_vec.insert(0, ".".repeat(new_vec[0].len()));
    new_vec.insert(new_vec.len(), ".".repeat(new_vec[0].len()));
    return new_vec;
}