use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;

use itertools::Itertools;

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

impl Point {
    pub fn distance(&self, other: &Self) -> f64 {
        ((self.x.abs_diff(other.x).pow(2)) as f64 + (self.y.abs_diff(other.y).pow(2)) as f64).sqrt()
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

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}, {})", self.x, self.y).as_str())
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            x => x,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn are_collinear(vec: Vec<&Point>) -> bool {
    if vec.len() < 3 {
        return true;
    }

    vec.iter().tuple_windows::<(_, _, _)>().all(|(p1, p2, p3)| {
        (p3.y as f64 - p2.y as f64) * (p2.x as f64 - p1.x as f64)
            == (p2.y as f64 - p1.y as f64) * (p3.x as f64 - p2.x as f64)
    })
}

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub struct PointI {
    pub x: isize,
    pub y: isize,
}

impl PointI {
    pub fn new(x: isize, y: isize) -> Self {
        PointI { x: x, y: y }
    }
}

impl PartialEq for PointI {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq<(isize, isize)> for PointI {
    fn eq(&self, other: &(isize, isize)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl Display for PointI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("({}, {})", self.x, self.y).as_str())
    }
}

impl Ord for PointI {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            x => x,
        }
    }
}

impl PartialOrd for PointI {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
