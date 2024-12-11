use itertools::Itertools;
use setup_utils::*;
use std::{collections::HashMap, path::Path};

// Symbols to replace: 11 55312 TEST2 186203 221291560078593

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/11-example.txt"));
        let result = crate::part1(&lines);
        if result == 55312 {
            Ok(())
        } else {
            Err(format!(
                "11: Bad result for Part 1 example, expected 55312 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/11-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (186203, 221291560078593) => Ok(()),
            (_, 221291560078593) => Err(format!(
                "11: Bad result for Part 1, expected 186203 got {}",
                result1
            )),
            (186203, _) => Err(format!(
                "11: Bad result for Part 2, expected 221291560078593 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "11: Bad result for Part 1 & 2, expected (186203, 221291560078593) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/11-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/11-example.txt"));

    println!("11-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("11-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

fn process_tree(cache: &mut HashMap<(u128, u8), u128>, num: u128, counter: u8) -> u128 {
    if cache.contains_key(&(num, counter)) {
        return cache.get(&(num, counter)).unwrap().clone();
    }

    if counter == 0 {
        return 1;
    }

    let res = match num {
        0 => process_tree(cache, 1, counter - 1),
        _ if (num.ilog10() + 1) % 2 == 0 => {
            let log = num.ilog10();
            let pow = 10u128.pow((log + 1) / 2);

            process_tree(cache, num / pow, counter - 1)
                + process_tree(cache, num % pow, counter - 1)
        }
        _ => process_tree(cache, num * 2024, counter - 1),
    };

    cache.insert((num, counter), res.clone());

    res
}

fn internal(lines: &Vec<String>, itercount: u8) -> u128 {
    let initial = lines[0]
        .split_ascii_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect_vec();

    let mut cache = HashMap::new();

    initial
        .iter()
        .map(|stone| process_tree(&mut cache, *stone, itercount))
        .sum()
}

fn part1(lines: &Vec<String>) -> u128 {
    internal(lines, 25)
}

fn part2(lines: &Vec<String>) -> u128 {
    internal(lines, 75)
}
