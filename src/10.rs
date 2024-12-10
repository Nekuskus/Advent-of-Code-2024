use itertools::Itertools;
use setup_utils::*;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

// Symbols to replace: 10 36 81 789 SOLVE2

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/10-example.txt"));
        let result = crate::part1(&lines);
        if result == 36 {
            Ok(())
        } else {
            Err(format!(
                "10: Bad result for Part 1 example, expected 36 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/10-example.txt"));
        let result = crate::part2(&lines);
        if result == 81 {
            Ok(())
        } else {
            Err(format!(
                "10: Bad result for Part 2 example, expected 81 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/10-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);

        if result1 == 789 {
            Ok(())
        } else {
            Err(format!(
                "10: Bad result for Part 1, expected 789 got {}",
                result1
            ))
        }
        /*
        match (result1, result2) {
            (789, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!(
                "10: Bad result for Part 1, expected 789 got {}",
                result1
            )),
            (789, _) => Err(format!(
                "10: Bad result for Part 2, expected SOLVE2 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "10: Bad result for Part 1 & 2, expected (789, SOLVE2) got ({}, {})",
                result1, result2
            )),
        }*/
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/10-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/10-example.txt"));

    println!("10-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("10-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

fn get_neighbors(matrix: &Vec<Vec<u32>>, p: &Point, step: u32) -> Vec<Point> {
    let mut ret = vec![];

    let xlen = matrix[0].len();
    let ylen = matrix.len();

    if p.x > 0 && matrix[p.y][p.x] + step == matrix[p.y][p.x - 1] {
        ret.push(Point::new(p.x - 1, p.y));
    }

    if p.x < xlen - 1 && matrix[p.y][p.x] + step == matrix[p.y][p.x + 1] {
        ret.push(Point::new(p.x + 1, p.y));
    }

    if p.y > 0 && matrix[p.y][p.x] + step == matrix[p.y - 1][p.x] {
        ret.push(Point::new(p.x, p.y - 1));
    }

    if p.y < ylen - 1 && matrix[p.y][p.x] + step == matrix[p.y + 1][p.x] {
        ret.push(Point::new(p.x, p.y + 1));
    }

    ret
}

fn search_trails(matrix: &Vec<Vec<u32>>, start: Point, step: u32) -> usize {
    let mut endpoints = HashSet::new();

    let mut queue = vec![(start, 0)];

    while let Some((p, cost_so_far)) = queue.pop() {
        if matrix[p.y][p.x] == 9 {
            endpoints.insert(p);
        }

        let neighbors = get_neighbors(matrix, &p, step)
            .iter()
            .map(|&p| (p, cost_so_far + 1))
            .collect_vec();

        queue.extend(neighbors);
    }

    endpoints.len()
}

fn part1(lines: &Vec<String>) -> usize {
    let matrix = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let trailheads = matrix
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(x, &d)| (d == 0).then_some(Point::new(x, y)))
        })
        .flatten();

    trailheads.map(|p| search_trails(&matrix, p, 1)).sum()
}

fn search_trails_p2(matrix: &Vec<Vec<u32>>, start: Point, step: u32) -> usize {
    let mut endpoints = HashMap::new();

    let mut queue = vec![(start, 0)];

    while let Some((p, cost_so_far)) = queue.pop() {
        if matrix[p.y][p.x] == 9 {
            match endpoints.get_mut(&p) {
                Some(v) => {
                    *v += 1;
                }
                None => {
                    endpoints.insert(p, 1);
                }
            }
        }

        let neighbors = get_neighbors(matrix, &p, step)
            .iter()
            .map(|&p| (p, cost_so_far + 1))
            .collect_vec();

        queue.extend(neighbors);
    }

    endpoints.values().sum()
}

fn part2(lines: &Vec<String>) -> usize {
    let matrix = lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let trailheads = matrix
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(x, &d)| (d == 0).then_some(Point::new(x, y)))
        })
        .flatten();

    trailheads.map(|p| search_trails_p2(&matrix, p, 1)).sum()
}
