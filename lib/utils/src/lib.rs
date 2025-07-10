
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fs::read_to_string;
use std::path::Path;

use itertools::Itertools;

// Reexport macro for glob import
extern crate macro_lib;
pub use macro_lib::get_input;

#[deprecated(note = "Replaced for input loading with get_input!()")]
pub fn read_lines(filename: &Path) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|s| s.trim().to_owned())
        .collect() // gather them together into a vector
}

#[deprecated(note = "Rewrite usage to check values instead")]
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

pub struct PriorityQueue<T> {
    queue: VecDeque<(T, i32)>
}

// Presumably O(log n) because of binary_search_by
#[allow(dead_code)]
impl<T> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue { queue: VecDeque::new() }
    }
    
    pub fn from(other: Vec<(T, i32)>) -> Self {
        let mut q = PriorityQueue { queue: VecDeque::new() };
        for (item, prio) in other {
            q.enqueue(item, prio)
        }
        q
    }

    pub fn enqueue(&mut self, item: T, priority: i32) {
        match self.queue.binary_search_by(|(_, prio)| prio.cmp(&priority)) {
            Ok(idx) | Err(idx) => {
                self.queue.insert(idx, (item, priority));
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if let Some(item) = self.queue.pop_front() {
            Some(item.0)
        } else {
            None
        }
    }
    
    pub fn dequeue_with_cost(&mut self) -> Option<(T, i32)> {
        self.queue.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(item) = self.queue.front() {
            Some(&item.0)
        } else {
            None
        }
    }

    pub fn length(&self) -> usize {
        self.queue.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

impl<T> Iterator for PriorityQueue<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.dequeue()
    }
}