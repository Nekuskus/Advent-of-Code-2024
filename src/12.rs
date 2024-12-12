use debug_print::{debug_print as debug, debug_println as debugln};
use itertools::Itertools;
use setup_utils::*;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    path::Path,
};

// Symbols to replace: 12 1930 1206 1387004 SOLVE2

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/12-example.txt"));
        let result = crate::part1(&lines);
        if result == 1930 {
            Ok(())
        } else {
            Err(format!(
                "12: Bad result for Part 1 example, expected 1930 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/12-example.txt"));
        let result = crate::part2(&lines);
        if result == 1206 {
            Ok(())
        } else {
            Err(format!(
                "12: Bad result for Part 2 example, expected 1206 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/12-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);

        if result1 == 1387004 {
            Ok(())
        } else {
            Err(format!(
                "12: Bad result for Part 1, expected 1387004 got {}",
                result1
            ))
        }
        /*
        match (result1, result2) {
            (1387004, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!(
                "12: Bad result for Part 1, expected 1387004 got {}",
                result1
            )),
            (1387004, _) => Err(format!(
                "12: Bad result for Part 2, expected SOLVE2 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "12: Bad result for Part 1 & 2, expected (1387004, SOLVE2) got ({}, {})",
                result1, result2
            )),
        }*/
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/12-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/12-example.txt"));

    println!("12-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("12-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

#[derive(Debug, PartialEq)]
enum Align {
    Up,
    Down,
    Left,
    Right,
    Both,
}

fn process_region(grid: &Vec<Vec<char>>, visited: &mut BTreeSet<PointI>, p: &PointI) -> usize {
    let mut area = BTreeSet::new();
    let mut perimeter = BTreeMap::new();

    let (xlen, ylen) = (grid[0].len() as isize, grid.len() as isize);
    let symbol = grid[p.y as usize][p.x as usize];

    let mut queue = VecDeque::from(vec![PointI::new(p.x as isize, p.y as isize)]);

    while let Some(next) = queue.pop_front() {
        area.insert(next);

        let neighbors = [
            PointI::new(next.x - 1, next.y),
            PointI::new(next.x + 1, next.y),
            PointI::new(next.x, next.y - 1),
            PointI::new(next.x, next.y + 1),
        ];

        for n in neighbors {
            if n.x == -1
                || n.y == -1
                || n.x == xlen
                || n.y == ylen
                || grid[n.y as usize][n.x as usize] != symbol
            {
                // println!("{symbol} {next}");
                let conv = PointI::new(n.x as isize, n.y as isize);
                match perimeter.get_mut(&conv) {
                    Some(count) => *count += 1,
                    None => {
                        perimeter.insert(conv, 1);
                    }
                }
            } else if !queue.contains(&n) && !area.contains(&n) {
                queue.push_front(n);
            }
        }
    }

    let ret = area.len() * perimeter.iter().map(|(_, v)| v).sum::<usize>();

    visited.extend(area);

    ret
}

fn part1(lines: &Vec<String>) -> usize {
    let matrix = lines.iter().map(|s| s.chars().collect_vec()).collect_vec();
    let mut visited = BTreeSet::new();

    let mut sum = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let p = PointI::new(x as isize, y as isize);
            if !visited.contains(&p) {
                sum += process_region(&matrix, &mut visited, &p);
            }
        }
    }

    sum
}

fn part2(lines: &Vec<String>) -> usize {
    let matrix = lines.iter().map(|s| s.chars().collect_vec()).collect_vec();

    let mut sum = 0;

    sum
}
