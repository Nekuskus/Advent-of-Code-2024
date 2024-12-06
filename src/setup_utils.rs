use std::fs::read_to_string;
use std::path::Path;

pub fn read_lines(filename: &Path) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|s| s.trim().to_owned())
        .collect() // gather them together into a vector
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn rotate_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    pub fn rotate_left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x: x, y: y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq<(usize, usize)> for Point {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}
