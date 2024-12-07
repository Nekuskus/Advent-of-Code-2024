use itertools::Itertools;
use setup_utils::*;
use std::{fmt::Display, path::Path};

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

impl Display for Ops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Add => "+",
            Self::Multiply => "*",
            Ops::Concat => "||",
        })
    }
}

#[allow(unused)]
fn process_ops(ops: Vec<&Ops>, operands: &Vec<i64>) -> i64 {
    // Slow, more straightforward implementation
    // Use by checking with target

    operands[1..]
        .iter()
        .enumerate()
        .fold(operands[0], |acc, (idx, &rhs)| match ops[idx] {
            Ops::Add => acc + rhs,
            Ops::Multiply => acc * rhs,
            Ops::Concat => acc * 10i64.pow(rhs.ilog10() + 1) + rhs,
        })
}

fn rprocess_ops(ops: Vec<&Ops>, operands: &Vec<i64>, target: i64) -> Option<()> {
    operands[1..]
        .iter()
        .enumerate()
        .try_rfold(target, |ret, (idx, &rhs)| match ops[ops.len() - idx - 1] {
            Ops::Add => Some(ret - rhs),
            Ops::Multiply => (ret % rhs == 0).then_some(ret / rhs),
            Ops::Concat => {
                let log = 10i64.pow(rhs.ilog10() + 1);

                (ret % log == rhs).then_some((ret - rhs) / log)
            }
        })
        .and_then(|res| (res == operands[0]).then_some(()))
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

            itertools::repeat_n(operations.iter(), operands.len() - 1) // permutation with replacements
                .multi_cartesian_product()
                .any(|ops| rprocess_ops(ops, &operands, lhs).is_some())
                .then_some(lhs)
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

            itertools::repeat_n(operations.iter(), operands.len() - 1) // permutation with replacements
                .multi_cartesian_product()
                .any(|ops| rprocess_ops(ops, &operands, lhs).is_some())
                .then_some(lhs)
        })
        .sum()
}
