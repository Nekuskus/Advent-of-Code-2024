use setup_utils::*;
use std::{path::Path, collections::HashSet};
use debug_print::{debug_print as debug, debug_println as debugln};

// Symbols to replace: 17 102 TEST2 SOLVE1 SOLVE2


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
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/17-2-example.txt"));
        let result = crate::part2(&lines);
        if result == TEST2 {
            Ok(())
        } else {
            Err(format!("17: Bad result for Part 2 example, expected TEST2 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/17-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);
        
        if result1 == SOLVE1 {
            Ok(())
        } else {
            Err(format!("17: Bad result for Part 1, expected SOLVE1 got {}", result1))
        }
        /*
        match (result1, result2) {
            (SOLVE1, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("17: Bad result for Part 1, expected SOLVE1 got {}", result1)),
            (SOLVE1, _) => Err(format!("17: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("17: Bad result for Part 1 & 2, expected (SOLVE1, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
    */
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/17-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/17-1-example.txt"));
    //let lines2 = read_lines(Path::new("./inputs/17-2-example.txt"));

    println!("17-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));
    
    println!("17-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
    
    
    //println!("17-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
    
}


fn part1(lines: &Vec<String>) -> i32 {
    use Direction::*;
    let charmap: Vec<Vec<i32>> = lines.iter().map(|s| s.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect()).collect();
    
    let mut to_visit: Vec<((usize, usize, Direction, u8), i32, HashSet<(usize, usize)>)> = vec![((1, 0, East, 1), 0, HashSet::new()), ((0, 1, South, 1), 0, HashSet::new())];
    //let mut found_costs: Vec<i32> = vec![];
    let mut found_min: i32 = i32::MAX;

    while let Some((node, mut total_cost, mut cache)) = to_visit.pop() {
        if cache.contains(&(node.0, node.1)) {
            continue;
        } else {
            cache.insert((node.0, node.1));
        }
        total_cost += charmap[node.1 as usize][node.0 as usize];
        //println!("{}", charmap[node.1 as usize][node.0 as usize]);
        if total_cost > found_min {
           continue; // worse than best currently found
        }
        //history.push((node.0, node.1));

        if (node.0, node.1) == (charmap[node.1].len() - 1, charmap.len() - 1) {
            if total_cost < found_min {
                found_min = total_cost; // Found exit
                println!("{node:?}, {total_cost}, len: {}", to_visit.len());
            } 
            continue;
        }
        //println!("{node:?} {total_cost}");
        match node.2 {
            North => {
                if node.3 < 3 && node.1 > 0 {
                    to_visit.push(((node.0, node.1 - 1, North, node.3 + 1), total_cost, cache.clone()));
                }
                if node.0 > 0 {
                    to_visit.push(((node.0 - 1, node.1, West, 0), total_cost, cache.clone()));
                }
                if node.0 < charmap[0].len() - 1 {
                    to_visit.push(((node.0 + 1, node.1, East, 0), total_cost, cache));
                }
            },
            South => {
                if node.0 > 0 {
                    to_visit.push(((node.0 - 1, node.1, West, 0), total_cost, cache.clone()));
                }
                if node.0 < charmap[0].len() - 1 {
                    to_visit.push(((node.0 + 1, node.1, East, 0), total_cost, cache.clone()));
                }
                if node.3 < 3 && node.1 < charmap.len() - 1 {
                    to_visit.push(((node.0, node.1 + 1, South, node.3 + 1), total_cost, cache));
                }
            },
            West => {
                if node.1 > 0 {
                    to_visit.push(((node.0, node.1 - 1, North, 0), total_cost, cache.clone()));
                }
                if node.3 < 3 && node.0 > 0 {
                    to_visit.push(((node.0 - 1, node.1, West, node.3 + 1), total_cost, cache.clone()));
                }
                if node.1 < charmap.len() - 1 {
                    to_visit.push(((node.0, node.1 + 1, South, 0), total_cost, cache));
                }
            }
            East => {
                if node.1 > 0 {
                    to_visit.push(((node.0, node.1 - 1, North, 0), total_cost, cache.clone()));
                }
                if node.3 < 3 && node.0 < charmap[0].len() - 1 {
                    to_visit.push(((node.0 + 1, node.1, East, node.3 + 1), total_cost, cache.clone()));
                }
                if node.1 < charmap.len() - 1 {
                    to_visit.push(((node.0, node.1 + 1, South, 0), total_cost, cache));
                }
            }
        }
    }
    //println!("{found_costs:?}");
    println!("{to_visit:?}");
    return found_min;
}
/*
fn part2(lines: &Vec<String>) -> i32 {

}
*/