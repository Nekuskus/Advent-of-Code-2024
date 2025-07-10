use std::{collections::{HashSet, HashMap}};
use setup_utils::*;
use std::{path::Path, collections::VecDeque};
use debug_print::{debug_print as debug, debug_println as debugln};

// Symbols to replace: 18 62 952408144115 70026 SOLVE2


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/18-example.txt"));
        let result = crate::part1(&lines);
        if result == 62 {
            Ok(())
        } else {
            Err(format!("18: Bad result for Part 1 example, expected 62 got {}", result))
        }
    }
    
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/18-example.txt"));
        let result = crate::part2(&lines);
        if result == 952408144115 {
            Ok(())
        } else {
            Err(format!("18: Bad result for Part 2 example, expected 952408144115 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/18-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);
        
        if result1 == 70026 {
            Ok(())
        } else {
            Err(format!("18: Bad result for Part 1, expected 70026 got {}", result1))
        }
        /*
        match (result1, result2) {
            (70026, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("18: Bad result for Part 1, expected 70026 got {}", result1)),
            (70026, _) => Err(format!("18: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("18: Bad result for Part 1 & 2, expected (70026, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/18-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/18-example.txt"));

    println!("18-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));
    
    println!("18-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));

}

// Up, Down, Right, Left
fn part1(lines: &Vec<String>) -> i32 {
    let mut grid = VecDeque::from([VecDeque::from([false; 1]); 1]);
    let start = Point::new(0, 0);
    let mut cur = start.clone();
    grid[start.y][start.x] = true;
    
    for line in lines {
        let strs: Vec<&str> = line.split_ascii_whitespace().collect();
        let count: i32 = strs[1].parse().unwrap();

        match strs[0] {
            "U" => {
                for i in 0..count {
                    if cur.y == 0 {
                        for _ in i..count { 
                            grid.push_front(VecDeque::from(vec![false].repeat(grid[0].len())));
                        }
                        cur.y += (count - i) as usize;
                    }
                    cur.y -= 1;
                    grid[cur.y][cur.x] = true;
                }
            },
            "D" => {
                for i in 0..count {
                    if cur.y == grid.len() - 1 {
                        for _ in i..count { 
                            grid.push_back(VecDeque::from(vec![false].repeat(grid[0].len())));
                        }
                    }
                    cur.y += 1;
                    grid[cur.y][cur.x] = true;
                }
            },
            "R" => {
                for i in 0..count {
                    if cur.x == grid[0].len() - 1 {
                        for _ in i..count { 
                            grid.iter_mut().for_each(|line| {
                                line.push_back(false)
                            });
                        }
                    }
                    cur.x += 1;
                    grid[cur.y][cur.x] = true;
                }
            },
            "L" => {
                for i in 0..count {
                    if cur.x == 0 {
                        for _ in i..count { 
                            grid.iter_mut().for_each(|line| {
                                line.push_front(false)
                            });
                        }
                        cur.x += (count - i) as usize;
                    }
                    cur.x -= 1;
                    grid[cur.y][cur.x] = true;
                }
            },
            _ => unreachable!()
        }
    }
    
    // Add outer layer for BFS to properly cover the area, use (0, 0)
    grid.push_front(VecDeque::from(vec![false].repeat(grid[0].len())));
    grid.push_back(VecDeque::from(vec![false].repeat(grid[0].len())));
    grid.iter_mut().for_each(|line| {
        line.push_front(false);
        line.push_back(false);
    });

    // BFS time!
    let mut visited: HashSet<Point> = HashSet::new();
    let mut to_visit = VecDeque::from(vec![start]);
    let mut bfs_markings = vec![vec![false; grid[0].len()]; grid.len()];


    while let Some(p) = to_visit.pop_front() {
        if visited.contains(&p) || grid[p.y][p.x] {
            continue;
        }

        if p.x > 0 {
            to_visit.push_back(Point::new(p.x - 1, p.y))
        }
        if p.x < grid[0].len() - 1 {
            to_visit.push_back(Point::new(p.x + 1, p.y))
        }
        if p.y > 0 {
            to_visit.push_back(Point::new(p.x, p.y - 1))
        }
        if p.y < grid.len() - 1 {
            to_visit.push_back(Point::new(p.x, p.y + 1))
        }
        bfs_markings[p.y][p.x] = true;
        visited.insert(p);
    }

    // Count border cells
    let perimeter_len = grid.iter().flatten().filter(|b| **b).count() as i32;
    let mut inner_count = 0;
    grid.iter_mut().enumerate().for_each(|(y, line)| {
        line.iter_mut().enumerate().for_each(|(x, item)| {
            if !*item && !bfs_markings[y][x] {
                inner_count += 1;
                *item = true;
            }
        })
    });

    

    for line in grid {
        debugln!("{}", line.iter().map(|&b| if b {'#'} else {'.'}).collect::<String>())
    }

    return perimeter_len +  inner_count;
}

fn part2(lines: &Vec<String>) -> i64 {
    //let mut grid = VecDeque::from([VecDeque::from([false; 1]); 1]);
    // key: line, value: list of horizontal lines, assume vertical lines unchanged
    let mut grid_lines: HashMap<i64, Vec<(i64, i64)>> = HashMap::new();
    let (mut x_pointer, mut y_pointer) = (0i64, 0i64);
    
    for line in lines {
        let strs: Vec<&str> = line.split_ascii_whitespace().collect();
        let inp = strs[2];
        let color = &inp[1..8];
        let dir = match color.chars().last().unwrap() {
            '0' => "R",
            '1' => "D",
            '2' => "L",
            '3' => "U",
            _ => unreachable!()
        };
        let count = i64::from_str_radix(&color[1..color.len()-1], 16).unwrap();
        debugln!("{dir} {count}");
        match dir {
            "U" => {
                y_pointer -= count;
            },
            "D" => {
                y_pointer += count;
            },
            "R" => {
                if grid_lines.contains_key(&y_pointer) {
                    grid_lines.get_mut(&y_pointer).unwrap().push((x_pointer, x_pointer + count));
                } else {
                    grid_lines.insert(y_pointer, vec![((x_pointer, x_pointer + count))]);
                }
                x_pointer += count;
            },
            "L" => {
                if grid_lines.contains_key(&y_pointer) {
                    grid_lines.get_mut(&y_pointer).unwrap().push((x_pointer, x_pointer - count));
                } else {
                    grid_lines.insert(y_pointer, vec![((x_pointer, x_pointer - count))]);
                }
                x_pointer -= count;
            },
            _ => unreachable!()
        }
    }
    let mut vec = grid_lines.iter().collect::<Vec<_>>();
    vec.sort_by(|a, b| a.0.cmp(b.0));
    for (key, line) in vec.clone() {
        println!("{key}: {line:?}");
    }

    let mut counter: i64 = 0;
    let mut line_iter = vec.iter();
    
    // These correspond to each other
    let mut pointers: Vec<(i64, i64)> = Vec::new();
    let mut prev_lens: Vec<i64> = Vec::new();
    let mut prev_line_num = i64::MIN;

    for (line_num, line) in line_iter {
        if prev_line_num == i64::MIN {
            let mut cloned = (*line).clone();
            let mut lens: Vec<i64> = line.iter().map(|(_x1, _x2)| _x2 - _x1 + 1).collect();
            lens.iter().for_each(|&length| {counter += length});
            //debugln!("{cloned:?}\n{lens:?}");
            prev_lens.append(&mut lens);
            pointers.append(&mut cloned);
            prev_line_num = **line_num;
        }
        for ((x1, x2), lines_count) in pointers.iter().zip(prev_lens.iter()) {

        }
        for (x1, x2) in line.clone() {
            let found_matches = pointers.iter().filter(|l|  (**l).0 == *x1 || (**l).1 == *x2).collect::<Vec<_>>();
            match found_matches.len() {
                2 => todo!(),
                1 => todo!(),
                0 => todo!(),
                _ => unreachable!()
            }
        }
    }

    return counter;
}
