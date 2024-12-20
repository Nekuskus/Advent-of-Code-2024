use setup_utils::*;
use std::{collections::HashSet, iter, path::Path};

// Symbols to replace: 06 41 6 4977 1729

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/06-example.txt"));
        let result = crate::part1(&lines);
        if result == 41 {
            Ok(())
        } else {
            Err(format!(
                "06: Bad result for Part 1 example, expected 41 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/06-example.txt"));
        let result = crate::part2(&lines);
        if result == 6 {
            Ok(())
        } else {
            Err(format!(
                "06: Bad result for Part 2 example, expected 6 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/06-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (4977, 1729) => Ok(()),
            (_, 1729) => Err(format!(
                "06: Bad result for Part 1, expected 4977 got {}",
                result1
            )),
            (4977, _) => Err(format!(
                "06: Bad result for Part 2, expected 1729 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "06: Bad result for Part 1 & 2, expected (4977, 1729) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/06-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/06-example.txt"));

    println!("06-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("06-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

fn matrix_step(
    matrix: &Vec<Vec<char>>,
    p: &Point,
    d: Direction,
    ignore: Option<Point>,
) -> Option<(Point, Direction)> {
    let xlen = matrix[0].len();
    let ylen = matrix.len();

    match d {
        Direction::North => {
            if p.y > 0 {
                if matrix[p.y - 1][p.x] != '#'
                    && (ignore.is_none() || ignore.unwrap() != (p.x, p.y - 1))
                {
                    Some((Point { x: p.x, y: p.y - 1 }, Direction::North))
                } else {
                    matrix_step(matrix, p, Direction::East, ignore)
                }
            } else {
                None
            }
        }
        Direction::East => {
            if p.x < xlen - 1 {
                if matrix[p.y][p.x + 1] != '#'
                    && (ignore.is_none() || ignore.unwrap() != (p.x + 1, p.y))
                {
                    Some((Point::new(p.x + 1, p.y), Direction::East))
                } else {
                    matrix_step(matrix, p, Direction::South, ignore)
                }
            } else {
                None
            }
        }
        Direction::South => {
            if p.y < ylen - 1 {
                if matrix[p.y + 1][p.x] != '#'
                    && (ignore.is_none() || ignore.unwrap() != (p.x, p.y + 1))
                {
                    Some((Point::new(p.x, p.y + 1), Direction::South))
                } else {
                    matrix_step(matrix, p, Direction::West, ignore)
                }
            } else {
                None
            }
        }
        Direction::West => {
            if p.x > 0 {
                if matrix[p.y][p.x - 1] != '#'
                    && (ignore.is_none() || ignore.unwrap() != (p.x - 1, p.y))
                {
                    Some((Point::new(p.x - 1, p.y), Direction::West))
                } else {
                    matrix_step(matrix, p, Direction::North, ignore)
                }
            } else {
                None
            }
        }
    }
}

fn part1(lines: &Vec<String>) -> usize {
    let mut matrix = lines
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut startx = None;
    let mut starty = None;
    'search: for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '^' {
                startx = Some(x);
                starty = Some(y);
                break 'search;
            }
        }
    }

    assert!(startx.is_some() && starty.is_some());

    let (mut p, mut d) = (
        Point {
            x: startx.unwrap(),
            y: starty.unwrap(),
        },
        Direction::North,
    );

    matrix[p.y][p.x] = 'X';

    while let Some((newp, newd)) = matrix_step(&matrix, &p, d, None) {
        p = newp;
        d = newd;
        matrix[p.y][p.x] = 'X';
    }

    matrix
        .iter()
        .map(|line| line.iter().filter(|&&c| c == 'X').count())
        .sum()
}

fn part2(lines: &Vec<String>) -> usize {
    let mut matrix = lines
        .iter()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut startx = None;
    let mut starty = None;

    'search: for (y, line) in matrix.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '^' {
                startx = Some(x);
                starty = Some(y);
                break 'search;
            }
        }
    }

    assert!(startx.is_some() && starty.is_some());

    let (p, d) = (
        Point {
            x: startx.unwrap(),
            y: starty.unwrap(),
        },
        Direction::North,
    );

    matrix[p.y][p.x] = 'X';

    let mut check_matrix = matrix.clone();
    let (mut loop_p, mut loop_d) = (p.clone(), d.clone());
    // first pass to find viable points
    while let Some((newp, newd)) = matrix_step(&check_matrix, &loop_p, loop_d, None) {
        loop_p = newp;
        loop_d = newd;
        check_matrix[loop_p.y][loop_p.x] = 'X';
    }

    let visited = check_matrix
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, &c)| {
                    if c == 'X' {
                        Some(Point::new(x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<Point>>()
        })
        .flatten()
        .collect::<HashSet<Point>>();

    iter::repeat_n(&matrix, visited.len())
        .zip(visited.iter())
        .filter(|(matrix, &ignore)| {
            if matrix[ignore.y][ignore.x] == 'X' {
                // don't replace starting position
                return false;
            }

            let (mut p, mut d) = (p.clone(), d.clone());

            let mut history = HashSet::new();
            history.insert((p, d));

            while let Some((newp, newd)) = matrix_step(&matrix, &p, d, Some(ignore)) {
                p = newp;
                d = newd;

                if history.contains(&(p, d)) {
                    // loop found
                    return true;
                }
                history.insert((p, d));
            }

            false
        })
        .count()
}
