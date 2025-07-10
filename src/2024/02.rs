#![feature(iter_map_windows)]

use itertools::Itertools;
use utils::*;

// Symbols to replace: 02 2 4 472 520

#[cfg(test)]
mod tests {
    use utils::get_input;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = get_input!("02-example.txt");
        let result = crate::part1(&lines);
        if result == 2 {
            Ok(())
        } else {
            Err(format!(
                "02: Bad result for Part 1 example, expected 2 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = get_input!("02-example.txt");
        let result = crate::part2(&lines);
        if result == 4 {
            Ok(())
        } else {
            Err(format!(
                "02: Bad result for Part 2 example, expected 4 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = get_input!("02-full.txt");
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (472, 520) => Ok(()),
            (_, 520) => Err(format!(
                "02: Bad result for Part 1, expected 472 got {}",
                result1
            )),
            (472, _) => Err(format!(
                "02: Bad result for Part 2, expected 520 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "02: Bad result for Part 1 & 2, expected (472, 520) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = get_input!("02-full.txt");
    let lines1 = get_input!("02-example.txt");

    println!("02-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("02-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

fn validate_report<T: Iterator<Item = i32> + Clone>(it: &T) -> bool {
    let inc = it.clone().map_windows(|&[x, y]| y - x).collect_vec();

    (inc.iter().all(|&x| x > 0) || inc.iter().all(|&x| x < 0))
        && inc.iter().all(|x| x.abs() >= 1 && x.abs() <= 3)
}

fn part1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
        })
        .filter(validate_report)
        .count()
}

fn validate_report_skip<T: Iterator<Item = i32> + Clone>(it: &T, skip: usize) -> bool {
    let inc = it
        .clone()
        .enumerate()
        .filter_map(|(idx, num)| (idx != skip).then_some(num))
        .map_windows(|&[x, y]| y - x);

    (inc.clone().all(|x| x > 0) || inc.clone().all(|x| x < 0))
        && inc.clone().all(|x| x.abs() >= 1 && x.abs() <= 3)
}

fn part2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
        })
        .filter(|parsed| {
            let base_vec = parsed.clone().collect::<Vec<i32>>();

            std::iter::repeat_n(base_vec.iter().copied(), base_vec.len())
                .enumerate()
                .any(|(idx, it)| validate_report_skip(&it, idx))
        })
        .count()
}
