use utils::*;
use std::{collections::HashMap, path::Path};

// Symbols to replace: 01 11 31 1666427 24316233

// #[cfg(test)]
// mod tests {
//     use utils::get_input;
//     use std::path::Path;

//     #[test]
//     fn part1() -> Result<(), String> {
//         let lines = get_input!("01-example.txt");
//         let result = crate::part1(&lines);
//         if result == 11 {
//             Ok(())
//         } else {
//             Err(format!(
//                 "01: Bad result for Part 1 example, expected 11 got {}",
//                 result
//             ))
//         }
//     }

//     #[test]
//     fn part2() -> Result<(), String> {
//         let lines = get_input!("01-example.txt");
//         let result = crate::part2(&lines);
//         if result == 31 {
//             Ok(())
//         } else {
//             Err(format!(
//                 "01: Bad result for Part 2 example, expected 31 got {}",
//                 result
//             ))
//         }
//     }

//     #[test]
//     fn full() -> Result<(), String> {
//         let lines = get_input!("01-full.txt");
//         let result1 = crate::part1(&lines);
//         let result2 = crate::part2(&lines);

//         match (result1, result2) {
//             (1666427, 24316233) => Ok(()),
//             (_, 24316233) => Err(format!(
//                 "01: Bad result for Part 1, expected 1666427 got {}",
//                 result1
//             )),
//             (1666427, _) => Err(format!(
//                 "01: Bad result for Part 2, expected 24316233 got {}",
//                 result2
//             )),
//             (_, _) => Err(format!(
//                 "01: Bad result for Part 1 & 2, expected (1666427, 24316233) got ({}, {})",
//                 result1, result2
//             )),
//         }
//     }
// }

fn main() {
    let linesfull = get_input!("01-full.txt");
    let lines1 = get_input!("01-example.txt");

    println!("01-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("01-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

fn part1(lines: &Vec<String>) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = lines
        .iter()
        .map(|s| s.split_ascii_whitespace())
        .map(|mut spl| {
            (
                spl.next().unwrap().parse::<u32>().unwrap(),
                spl.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (&l, &r)| acc + l.abs_diff(r))
}

fn part2(lines: &Vec<String>) -> u32 {
    let (left, right): (Vec<u32>, Vec<u32>) = lines
        .iter()
        .map(|s| s.split_ascii_whitespace())
        .map(|mut spl| {
            (
                spl.next().unwrap().parse::<u32>().unwrap(),
                spl.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip();

    let mut hashmap: HashMap<u32, u32> = HashMap::new();

    for r in right {
        match hashmap.get_mut(&r) {
            Some(val) => {
                *val += 1;
            }
            None => {
                hashmap.insert(r, 1);
            }
        }
    }

    left.iter()
        .fold(0, |acc, l| acc + l * hashmap.get(l).unwrap_or(&0))
}
