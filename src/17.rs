use setup_utils::*;
use std::{path::Path, collections::HashMap};
use debug_print::{debug_print as debug, debug_println as debugln};

// Symbols to replace: 17 102 71 767 904


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/17-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 102 {
            Ok(())
        } else {
            Err(format!("17: Bad result for Part 1 example, expected 102 got {}", result))
        }
    }
    
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/17-2-example.txt"));
        let result = crate::part2(&lines);
        if result == 71 {
            Ok(())
        } else {
            Err(format!("17: Bad result for Part 2 example, expected 71 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/17-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (767, 904) => Ok(()),
            (_, 904) => Err(format!("17: Bad result for Part 1, expected 767 got {}", result1)),
            (767, _) => Err(format!("17: Bad result for Part 2, expected 904 got {}", result2)),
            (_, _) => Err(format!("17: Bad result for Part 1 & 2, expected (767, 904) got ({}, {})", result1, result2))
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/17-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/17-1-example.txt"));
    let lines2 = read_lines(Path::new("./inputs/17-2-example.txt"));

    println!("17-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));
    
    println!("17-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
    
    
    println!("17-2-example.txt");
    println!("{}", part1(&lines2));
    println!("{}", part2(&lines2));
    
}

fn neighbours(grid: &Vec<Vec<i32>>, x: usize, y: usize, dir: Direction, counter: u8) -> Vec<((usize, usize, Direction, u8), i32)>{
    use Direction::*;
    let mut ret = Vec::new();
    if y > 0 && dir != South {
        if dir == North {
            if counter != 3 {
                ret.push(((x, y - 1, North, counter + 1), grid[y - 1][x]));
            } // else push with cost = i32::MAX, unnecessary
        } else {
            ret.push(((x, y - 1, North, 1), grid[y - 1][x]));
        }
    }
    if y < grid.len() - 1 && dir != North {
        if dir == South {
            if counter != 3 {
                ret.push(((x, y + 1, South, counter + 1), grid[y + 1][x]));
            }
        } else {
            ret.push(((x, y + 1, South, 1), grid[y + 1][x]));
        }
    }
    if x > 0 && dir != East {
        if dir == West {
            if counter != 3 {
                ret.push(((x - 1, y, West, counter + 1), grid[y][x - 1]));
            }
        } else {
            ret.push(((x - 1, y, West, 1), grid[y][x - 1]));
        }
    }
    if x < grid[y].len() - 1 && dir != West {
        if dir == East {
            if counter != 3 {
                ret.push(((x + 1, y, East, counter + 1), grid[y][x + 1]));
            }
        } else {
            ret.push(((x + 1, y, East, 1), grid[y][x + 1]));
        }
    }
    ret
}

fn part1(lines: &Vec<String>) -> i32 {
    use Direction::*;
    let tiles = lines.len() * lines[0].len()  * 4; // because 4 directions
    let charmap: Vec<Vec<i32>> = lines.iter().map(|s| s.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect()).collect();
    let mut came_from: HashMap<(usize, usize, Direction, u8), (usize, usize, Direction, u8)> = HashMap::new();
    let mut cost_so_far: HashMap<(usize, usize, Direction, u8), i32> = HashMap::new();
    cost_so_far.insert((0, 0, North, 0), 0);

    let mut frontier: PriorityQueue<(usize, usize, Direction, u8)> = PriorityQueue::from(vec![((0, 0, North, 0), 0)]);
    let goal = (charmap[charmap.len() - 1].len() - 1, charmap.len() - 1);

    //println!("{:?}", neighbours(&charmap, 0, 0, North, 0));
    let mut found_cost: i32 = -1;
    let mut last = (0, 0, North, 0);
    // Priority within queue is treated as current cost

    while let Some(((x, y, dir, counter), cost)) = frontier.dequeue_with_cost() {
        if (x, y) == goal {
            found_cost = cost;
            last = (x, y, dir, counter);
            break;
        }

        for ((nx, ny, ndir, ncounter), ncost) in neighbours(&charmap, x, y, dir, counter) {
            let cur_cost = cost_so_far.get(&(x, y, dir, counter)).unwrap();
            let new_cost = cur_cost + ncost;
            
            if !cost_so_far.contains_key(&(nx, ny, ndir, ncounter)) || new_cost < *cost_so_far.get(&(nx, ny, ndir, ncounter)).unwrap() {
                cost_so_far.insert((nx, ny, ndir, ncounter), new_cost);
                frontier.enqueue((nx, ny, ndir, ncounter), new_cost);
                came_from.insert((nx, ny, ndir, ncounter), (x, y, dir, counter));
            }
        }
    }
    let mut printable = charmap.clone();

    let mut cur = last;
    loop {
        printable[cur.1][cur.0] = -1;
        cur = match came_from.get(&cur) {
            Some(node) => *node,
            None => { break; }
        };
        //debug!("{cur:?}")
    }

    for line in printable {
        for c in line {
            debug!("{}", if c != -1 { c.to_string() } else { "#".to_string() })
        }
        debugln!()
    }

    return found_cost;
}

fn neighbours_part2(grid: &Vec<Vec<i32>>, x: usize, y: usize, dir: Direction, counter: u8) -> Vec<((usize, usize, Direction, u8), i32)>{
    use Direction::*;
    let mut ret = Vec::new();
    if y > 0 && dir != South {
        if dir == North {
            if counter != 10 {
                ret.push(((x, y - 1, North, counter + 1), grid[y - 1][x]));
            } // else push with cost = i32::MAX, unnecessary
        } else if dir != North && counter >= 4 {
            ret.push(((x, y - 1, North, 1), grid[y - 1][x]));
        }
    }
    if y < grid.len() - 1 && dir != North {
        if dir == South {
            if counter != 10 {
                ret.push(((x, y + 1, South, counter + 1), grid[y + 1][x]));
            }
        } else if dir != South && counter >= 4 {
            ret.push(((x, y + 1, South, 1), grid[y + 1][x]));
        }
    }
    if x > 0 && dir != East {
        if dir == West {
            if counter != 10 {
                ret.push(((x - 1, y, West, counter + 1), grid[y][x - 1]));
            }
        } else if dir != West && counter >= 4 {
            ret.push(((x - 1, y, West, 1), grid[y][x - 1]));
        }
    }
    if x < grid[y].len() - 1 && dir != West {
        if dir == East {
            if counter != 10 {
                ret.push(((x + 1, y, East, counter + 1), grid[y][x + 1]));
            }
        } else if dir != East && counter >= 4 {
            ret.push(((x + 1, y, East, 1), grid[y][x + 1]));
        }
    }
    ret
}

fn part2(lines: &Vec<String>) -> i32 {
    use Direction::*;
    let tiles = lines.len() * lines[0].len()  * 4; // because 4 directions
    let charmap: Vec<Vec<i32>> = lines.iter().map(|s| s.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect()).collect();
    let mut came_from: HashMap<(usize, usize, Direction, u8), (usize, usize, Direction, u8)> = HashMap::new();
    let mut cost_so_far: HashMap<(usize, usize, Direction, u8), i32> = HashMap::new();
    cost_so_far.insert((0, 0, North, 0), 0);
    cost_so_far.insert((0, 1, South, 1), charmap[1][0]);
    cost_so_far.insert((1, 0, East, 1), charmap[0][1]);

    let mut frontier: PriorityQueue<(usize, usize, Direction, u8)> = PriorityQueue::from(vec![((0, 1, South, 1), charmap[1][0]), ((1, 0, East, 1), charmap[0][1])]);
    let goal = (charmap[charmap.len() - 1].len() - 1, charmap.len() - 1);

    //println!("{:?}", neighbours(&charmap, 0, 0, North, 0));
    let mut found_cost: i32 = -1;
    let mut last = (0, 0, North, 0);
    // Priority within queue is treated as current cost

    while let Some(((x, y, dir, counter), cost)) = frontier.dequeue_with_cost() {
        if (x, y) == goal && counter >= 4 {
            found_cost = cost;
            last = (x, y, dir, counter);
            break;
        }

        for ((nx, ny, ndir, ncounter), ncost) in neighbours_part2(&charmap, x, y, dir, counter) {
            let cur_cost = cost_so_far.get(&(x, y, dir, counter)).unwrap();
            let new_cost = cur_cost + ncost;
            
            if !cost_so_far.contains_key(&(nx, ny, ndir, ncounter)) || new_cost < *cost_so_far.get(&(nx, ny, ndir, ncounter)).unwrap() {
                cost_so_far.insert((nx, ny, ndir, ncounter), new_cost);
                frontier.enqueue((nx, ny, ndir, ncounter), new_cost);
                came_from.insert((nx, ny, ndir, ncounter), (x, y, dir, counter));
            }
        }
    }
    let mut printable = charmap.clone();

    let mut cur = last;
    loop {
        printable[cur.1][cur.0] = -1;
        cur = match came_from.get(&cur) {
            Some(node) => *node,
            None => { break; }
        };
        //debug!("{cur:?}")
    }

    for line in printable {
        for c in line {
            debug!("{}", if c != -1 { c.to_string() } else { "#".to_string() })
        }
        debugln!()
    }

    return found_cost;
}
