use setup_utils::*;
use std::path::Path;
use std::collections::HashMap;
use debug_print::debug_println as debugln;

// Symbols to replace: 14 136 64 109939 101010

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/14-example.txt"));
        let result = crate::part1(&lines);
        if result == 136 {
            Ok(())
        } else {
            Err(format!(
                "14: Bad result for Part 1 example, expected 136 got {}",
                result
            ))
        }
    }
    
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/14-example.txt"));
        let result = crate::part2(&lines);
        if result == 64 {
            Ok(())
        } else {
            Err(format!("14: Bad result for Part 2 example, expected 64 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/14-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (109939, 101010) => Ok(()),
            (_, 101010) => Err(format!("14: Bad result for Part 1, expected 109939 got {}", result1)),
            (109939, _) => Err(format!("14: Bad result for Part 2, expected 101010 got {}", result2)),
            (_, _) => Err(format!("14: Bad result for Part 1 & 2, expected (109939, 101010) got ({}, {})", result1, result2))
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/14-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/14-example.txt"));

    println!("14-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("14-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn tilt(table: &mut Vec<Vec<char>>, dir: Direction) {
    loop {
        for y in 0..table.len() {
            for x in 0..table[y].len() {
                match dir {
                    Direction::North => {
                        if y > 0 && table[y][x] == 'O' && table[y - 1][x] == '.' {
                            table[y][x] = '.';
                            table[y - 1][x] = 'O';
                        }
                    }
                    Direction::East => {
                        if x < table[y].len() - 1 && table[y][x] == 'O' && table[y][x + 1] == '.' {
                            table[y][x] = '.';
                            table[y][x + 1] = 'O';
                        }
                    }
                    Direction::South => {
                        if y < table.len() - 1 && table[y][x] == 'O' && table[y + 1][x] == '.' {
                            table[y][x] = '.';
                            table[y + 1][x] = 'O';
                        }
                    }
                    Direction::West => {
                        if x > 0 && table[y][x] == 'O' && table[y][x - 1] == '.' {
                            table[y][x] = '.';
                            table[y][x - 1] = 'O';
                        }
                    }
                }
            }
        }
        //for line in table.iter().as_ref() {
            //debugln!("{}", line.iter().collect::<String>());
        //}
        if (0..table.len()).all(|y| {
            (0..table[y].len()).all(|x| {
                if table[y][x] == 'O' {
                    match dir {
                        Direction::North => {
                            y == 0 || table[y - 1][x] == '#' || table[y - 1][x] == 'O' // stacked properly
                        }
                        Direction::East => {
                            x == table[y].len() - 1 || table[y][x + 1] == '#' || table[y][x + 1] == 'O'
                        }
                        Direction::South => {
                            y == table.len() - 1 || table[y + 1][x] == '#' || table[y + 1][x] == 'O'
                        }
                        Direction::West => {
                            x == 0 || table[y][x - 1] == '#' || table[y][x - 1] == 'O'
                        }
                    }
                } else { // '.' || '#'
                    return true;
                }
            })
        }) {
            break;
        }
    }
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut cloned: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();

    tilt(&mut cloned, Direction::North);

    // Calculate result
    cloned.iter().rev().enumerate().fold(0, |acc, (idx, line)| {
        //debugln!("{} {}", idx + 1, line.iter().collect::<String>());
        acc + (line.iter().filter(|c| c == &&'O').count()) as i32 * (idx + 1) as i32
    })
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut cache: HashMap<Vec<Vec<char>>, i32> = HashMap::new();
    let mut cloned: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    
    for i in 1..=1000000000 {
        tilt(&mut cloned, Direction::North);
        tilt(&mut cloned, Direction::West);
        tilt(&mut cloned, Direction::South);
        tilt(&mut cloned, Direction::East);
        if cache.contains_key(&cloned) {
            debugln!("Found cycle {}, for i: {i}", cache.get(&cloned).unwrap());
            break;
        } else {
            cache.insert(cloned.clone(), i);
        }
    }

    // Cycle found
    let cycle_length = cache.len();
    let cycles_left = (1000000000 % cycle_length) + 1;
    let mut cycle_sorted = cache.values().collect::<Vec<_>>().clone();
    cycle_sorted.sort();
    debugln!("Cycle length: {cycle_length} {:?}", cycle_sorted);
    debugln!("Cycles left: {cycles_left}");


    // Iterate remainder after cycle
    for _ in 0..cycles_left+1 {
        tilt(&mut cloned, Direction::North);
        tilt(&mut cloned, Direction::West);
        tilt(&mut cloned, Direction::South);
        tilt(&mut cloned, Direction::East);
    }

    // Calculate result
    cloned.iter().rev().enumerate().fold(0, |acc, (idx, line)| {
        //debugln!("{} {}", idx + 1, line.iter().collect::<String>());
        acc + (line.iter().filter(|c| c == &&'O').count()) as i32 * (idx + 1) as i32
    })
}

