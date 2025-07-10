use std::fs::read_to_string;
use std::{path::Path, collections::VecDeque};

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

// Too many problems required this one to not extract it
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

// Not sure if this one is required after day 17, but I'd rather have it extracted here
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
        len!(self.queue)
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