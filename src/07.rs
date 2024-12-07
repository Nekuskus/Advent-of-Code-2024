use itertools::Itertools;
use setup_utils::*;
use std::path::Path;

// Symbols to replace: 07 3749 11387 1399219271639 275791737999003

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/07-example.txt"));
        let result = crate::part1(&lines);
        if result == 3749 {
            Ok(())
        } else {
            Err(format!(
                "07: Bad result for Part 1 example, expected 3749 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/07-example.txt"));
        let result = crate::part2(&lines);
        if result == 11387 {
            Ok(())
        } else {
            Err(format!(
                "07: Bad result for Part 2 example, expected 11387 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/07-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (1399219271639, 275791737999003) => Ok(()),
            (_, 275791737999003) => Err(format!(
                "07: Bad result for Part 1, expected 1399219271639 got {}",
                result1
            )),
            (1399219271639, _) => Err(format!(
                "07: Bad result for Part 2, expected 275791737999003 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "07: Bad result for Part 1 & 2, expected (1399219271639, 275791737999003) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/07-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/07-example.txt"));

    println!("07-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("07-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Ops {
    Add,
    Multiply,
    Concat,
}

fn process_ops(ops: Vec<&Ops>, operands: &Vec<i64>) -> i64 {
    operands[1..]
        .iter()
        .enumerate()
        .fold(operands[0], |acc, (idx, &num)| match ops[idx] {
            Ops::Add => acc + num,
            Ops::Multiply => acc * num,
            Ops::Concat => acc * 10i64.pow(num.ilog10() + 1) + num,
        })
}

fn part1(lines: &Vec<String>) -> i64 {
    let operations = [Ops::Add, Ops::Multiply];

    lines
        .iter()
        .filter_map(|l| {
            let mut spl = l.split(":");
            let lhs = spl.next().unwrap().parse::<i64>().unwrap();
            let operands = spl
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec();

            if itertools::repeat_n(operations.iter(), operands.len() - 1) // permutation with replacements
                .multi_cartesian_product()
                .any(|ops| process_ops(ops, &operands) == lhs)
            {
                Some(lhs)
            } else {
                None
            }
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> i64 {
    let operations = [Ops::Add, Ops::Multiply, Ops::Concat];

    lines
        .iter()
        .filter_map(|l| {
            let mut spl = l.split(":");
            let lhs = spl.next().unwrap().parse::<i64>().unwrap();
            let operands = spl
                .next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec();

            if itertools::repeat_n(operations.iter(), operands.len() - 1) // permutation with replacements
                .multi_cartesian_product()
                .any(|ops| process_ops(ops, &operands) == lhs)
            {
                Some(lhs)
            } else {
                None
            }
        })
        .sum()
}
