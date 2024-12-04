use regex::Regex;
use setup_utils::*;
use std::path::Path;

// Symbols to replace: 03 161 48 185797128 89798695

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/03-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 161 {
            Ok(())
        } else {
            Err(format!(
                "03: Bad result for Part 1 example, expected 161 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/03-2-example.txt"));
        let result = crate::part2(&lines);
        if result == 48 {
            Ok(())
        } else {
            Err(format!(
                "03: Bad result for Part 2 example, expected 48 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/03-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (185797128, 89798695) => Ok(()),
            (_, 89798695) => Err(format!(
                "03: Bad result for Part 1, expected 185797128 got {}",
                result1
            )),
            (185797128, _) => Err(format!(
                "03: Bad result for Part 2, expected 89798695 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "03: Bad result for Part 1 & 2, expected (185797128, 89798695) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/03-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/03-1-example.txt"));
    let lines2 = read_lines(Path::new("./inputs/03-2-example.txt"));

    println!("03-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("03-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));

    println!("03-2-example.txt");
    println!("{}", part1(&lines2));
    println!("{}", part2(&lines2));
}

fn part1(lines: &Vec<String>) -> i32 {
    let re = Regex::new(r"mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap();

    lines
        .iter()
        .map(|line| {
            re.captures_iter(&line)
                .map(|caps| {
                    caps.name("first").unwrap().as_str().parse::<i32>().unwrap()
                        * caps
                            .name("second")
                            .unwrap()
                            .as_str()
                            .parse::<i32>()
                            .unwrap()
                })
                .sum::<i32>()
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> i32 {
    // look-behinds not supported by crate

    let mut enabled = true;
    let remul =
        Regex::new(r"do\(\)|don't\(\)|mul\((?<first>\d{1,3}),(?<second>\d{1,3})\)").unwrap();

    lines
        .iter()
        .map(|line| {
            remul
                .captures_iter(&line)
                .map(|caps| match &caps[0] {
                    "do()" => {
                        enabled = true;
                        0
                    }
                    "don't()" => {
                        enabled = false;
                        0
                    }
                    _ => {
                        if enabled {
                            caps.name("first").unwrap().as_str().parse::<i32>().unwrap()
                                * caps
                                    .name("second")
                                    .unwrap()
                                    .as_str()
                                    .parse::<i32>()
                                    .unwrap()
                        } else {
                            0
                        }
                    }
                })
                .sum::<i32>()
        })
        .sum()
}
