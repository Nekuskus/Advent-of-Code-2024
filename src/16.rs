use setup_utils::*;
use std::{path::Path, collections::HashSet};
use debug_print::debug_println as debugln;

// Symbols to replace: 16 46 51 7498 7846


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/16-example.txt"));
        let result = crate::part1(&lines);
        if result == 46 {
            Ok(())
        } else {
            Err(format!("16: Bad result for Part 1 example, expected 46 got {}", result))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/16-example.txt"));
        let result = crate::part2(&lines);
        if result == 51 {
            Ok(())
        } else {
            Err(format!("16: Bad result for Part 2 example, expected 51 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/16-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (7498, 7846) => Ok(()),
            (_, 7846) => Err(format!("16: Bad result for Part 1, expected 7498 got {}", result1)),
            (7498, _) => Err(format!("16: Bad result for Part 2, expected 7846 got {}", result2)),
            (_, _) => Err(format!("16: Bad result for Part 1 & 2, expected (7498, 7846) got ({}, {})", result1, result2))
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/16-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/16-example.txt"));

    println!("16-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));
    
    println!("16-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
}

fn energize(grid: &Vec<Vec<char>>, entry: (usize, usize, Direction)) -> i32 {
    use Direction::*;
    let mut energized: Vec<Vec<bool>> = grid.iter().map(|line| line.iter().map(|_| false).collect()).collect();
    let mut queue: Vec<(usize, usize, Direction)> = vec![entry]; // (x, y, dir)
    let mut cacheset: HashSet<(usize, usize, Direction)> = HashSet::new();


    while let Some(p) = queue.pop() {
        if cacheset.contains(&p) {
            continue;
        } else {
            cacheset.insert(p);
        }
        //println!("{queue:?}");
        //for line in &energized {
            //println!("{}", line.iter().map(|x| if *x { '#' } else { '.' }).collect::<String>())
        //}
        //println!("----------");
        energized[p.1][p.0] = true;
        match grid[p.1][p.0] {
            '.' => {
                match p.2 {
                    North => {
                        if p.1 > 0 {
                            queue.push((p.0, p.1 - 1, North));
                        }
                    },
                    South => {
                        if p.1 < grid.len() - 1 {
                            queue.push((p.0, p.1 + 1, South));
                        }
                    },
                    West => {
                        if p.0 > 0 {
                            queue.push((p.0 - 1, p.1, West));
                        }
                    },
                    East => {
                        if p.0 < grid[p.1].len() - 1 {
                            queue.push((p.0 + 1, p.1, East));
                        }
                    },
                }
            },
            '\\' => {
                match p.2 {
                    North => {
                        if p.0 > 0 {
                            queue.push((p.0 - 1, p.1, West));
                        }
                    },
                    South => {
                        if p.0 < grid[p.1].len() - 1 {
                            queue.push((p.0 + 1, p.1, East));
                        }
                    },
                    West => {
                        if p.1 > 0 {
                            queue.push((p.0, p.1 - 1, North));
                        }
                    },
                    East => {
                        if p.1 < grid.len() - 1 {
                            queue.push((p.0, p.1 + 1, South));
                        }
                    },
                }
            },
            '/' => {
                match p.2 {
                    North => {
                        if p.0 < grid[p.1].len() - 1 {
                            queue.push((p.0 + 1, p.1, East));
                        }
                    },
                    South => {
                        if p.0 > 0 {
                            queue.push((p.0 - 1, p.1, West));
                        }
                    },
                    West => {
                        if p.1 < grid.len() - 1 {
                            queue.push((p.0, p.1 + 1, South));
                        }
                    },
                    East => {
                        if p.1 > 0 {
                            queue.push((p.0, p.1 - 1, North));
                        }
                    }
                }
            }
            '|' => {
                match p.2 {
                    North => {
                        if p.1 > 0 {
                            queue.push((p.0, p.1 - 1, North));
                        }
                    },
                    South => {
                        if p.1 < grid.len() - 1 {
                            queue.push((p.0, p.1 + 1, South));
                        }
                    },
                    West | East => {
                        if p.1 > 0 {
                            queue.push((p.0, p.1 - 1, North));
                        }
                        if p.1 < grid.len() - 1 {
                            queue.push((p.0, p.1 + 1, South));
                        }
                    }
                }
            }
            '-' => {
                match p.2 {
                    North | South => {
                        if p.0 > 0 {
                            queue.push((p.0 - 1, p.1, West));
                        }
                        if p.0 < grid[p.1].len() - 1 {
                            queue.push((p.0 + 1, p.1, East));
                        }
                    },
                    West => {
                        if p.0 > 0 {
                            queue.push((p.0 - 1, p.1, West));
                        }
                    },
                    East => {
                        if p.0 < grid[p.1].len() - 1 {
                            queue.push((p.0 + 1, p.1, East));
                        }
                    },
                }
            }
            _ => unreachable!()
        }

    }

    return energized.iter().flatten().filter(|cell| **cell).count() as i32;
}

fn part1(lines: &Vec<String>) -> i32 {
    use Direction::*;
    let grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    return energize(&grid, (0, 0, East));
}

fn part2(lines: &Vec<String>) -> i32 {
    use Direction::*;
    let grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    let list_of_attempts = (0..grid[0].len()).map(|n| vec![(n, 0, South), (n, grid.len() - 1, North)]).chain((0..grid.len()).map(|n| vec![(0, n, West), (grid[n].len() - 1, n, East)])).flatten();
    debugln!("{:#?}", list_of_attempts.clone().collect::<Vec<_>>());
    return list_of_attempts.map(|item| energize(&grid, item)).max().unwrap();
}
