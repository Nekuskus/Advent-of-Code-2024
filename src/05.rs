use setup_utils::*;
use std::cmp::Ordering;
use std::{collections::HashMap, path::Path};

// Symbols to replace: 05 143 123 6051 5093

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-example.txt"));
        let result = crate::part1(&lines);
        if result == 143 {
            Ok(())
        } else {
            Err(format!(
                "05: Bad result for Part 1 example, expected 143 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-example.txt"));
        let result = crate::part2(&lines);
        if result == 123 {
            Ok(())
        } else {
            Err(format!(
                "05: Bad result for Part 2 example, expected 123 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (6051, 5093) => Ok(()),
            (_, 5093) => Err(format!(
                "05: Bad result for Part 1, expected 6051 got {}",
                result1
            )),
            (6051, _) => Err(format!(
                "05: Bad result for Part 2, expected 5093 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "05: Bad result for Part 1 & 2, expected (6051, 5093) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/05-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/05-example.txt"));

    println!("05-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("05-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut spl = lines.split(|l| l.trim().is_empty());
    let (rules_strings, lists_string) = (spl.next().unwrap(), spl.next().unwrap());

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    rules_strings.iter().for_each(|s| {
        let mut nums = s.split("|");
        let num = nums.next().unwrap().parse().unwrap();
        let requirement = nums.next().unwrap().parse().unwrap();
        match rules.get_mut(&num) {
            Some(entry) => {
                entry.push(requirement);
            }
            None => {
                rules.insert(num, vec![requirement]);
            }
        }
    });

    let lists = lists_string
        .iter()
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    lists
        .iter()
        .filter(|line| {
            line.iter()
                .enumerate()
                .all(|(idx_num, num)| match rules.get(num) {
                    Some(reqs) => reqs
                        .iter()
                        .all(|req| match line.iter().position(|x| x == req) {
                            Some(idx_req) => idx_req > idx_num,
                            None => true,
                        }),
                    None => true,
                })
        })
        .map(|line| {
            // println!("{line:?}: {}", line.len() / 2);
            line[line.len() / 2]
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut spl = lines.split(|l| l.trim().is_empty());
    let (rules_strings, lists_string) = (spl.next().unwrap(), spl.next().unwrap());

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    rules_strings.iter().for_each(|s| {
        let mut nums = s.split("|");
        let num = nums.next().unwrap().parse().unwrap();
        let requirement = nums.next().unwrap().parse().unwrap();
        match rules.get_mut(&num) {
            Some(entry) => {
                entry.push(requirement);
            }
            None => {
                rules.insert(num, vec![requirement]);
            }
        }
    });

    let mut lists = lists_string
        .iter()
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    lists
        .iter_mut()
        .filter(|line| {
            !line
                .iter()
                .enumerate()
                .all(|(idx_num, num)| match rules.get(num) {
                    Some(reqs) => reqs
                        .iter()
                        .all(|req| match line.iter().position(|x| x == req) {
                            Some(idx_req) => idx_req > idx_num,
                            None => true,
                        }),
                    None => true,
                })
        })
        .map(|line| {
            line.sort_by(|a, b| match rules.get(a) {
                Some(reqs) => {
                    if reqs.contains(b) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
                None => Ordering::Equal,
            });
            line
        })
        .map(|line| line[line.len() / 2])
        .sum()
}
