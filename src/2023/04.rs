use setup_utils::{len, read_lines};
use std::collections::HashSet;
use std::path::Path;

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/04-example.txt"));
        let result = crate::part1(&lines);
        if result == 13 {
            Ok(())
        } else {
            Err(format!(
                "04: Bad result for Part 1 example, expected 13 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/04-example.txt"));
        let result = crate::part2(&lines);
        if result == 30 {
            Ok(())
        } else {
            Err(format!(
                "04: Bad result for Part 2 example, expected 30 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/04-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);
        match (result1, result2) {
            (32609, 14624680) => Ok(()),
            (_, 14624680) => Err(format!(
                "04: Bad result for Part 1, expected 32609 got {}",
                result1
            )),
            (525181, _) => Err(format!(
                "04: Bad result for Part 2, expected 14624680 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "04: Bad result for Part 1 & 2, expected (32609, 14624680) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/04-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/04-example.txt"));

    println!("04-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("04-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut sum_of_points = 0;
    for line in lines {
        let mut nums_line = line.split(":").nth(1).expect(format!("Improperly formatted line: {}", line).as_str())
            .trim()
            .split('|')
            .map(|s| s.trim());
        let winning = nums_line.next().expect(format!("Improperly formatted line, missing winning numbers: {}", line).as_str())
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<i32>().expect(&format!("bad int error num={}", s)))
            .collect::<HashSet<_>>();
        let scratched = nums_line.next().expect(format!("Improperly formatted line, missing scratched numbers: {}", line).as_str())
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().expect(&format!("bad int error num={}", s)))
            .collect::<HashSet<_>>();
        let found = &winning & &scratched;
        if len!(found) > 0 {
            let score = 2_i32.pow(len!(found) as u32 - 1);
            sum_of_points += score;
        }
    }
    return sum_of_points;
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut total_count = 0;
    let lines_parsed = lines.iter().map(|line| {
        let mut split_line = line.split(":");
        let game_id = split_line.next().expect(format!("Improperly formatted line, missing game_id: {}", line).as_str())
            .split_ascii_whitespace()
            .nth(1).unwrap()
            .parse::<usize>().expect(&format!("bad int error num={}", line));
        let mut nums_line = split_line.next().expect(format!("Improperly formatted line: {}", line).as_str())
            .trim()
            .split('|')
            .map(|s| s.trim());
        let winning = nums_line.next().expect(format!("Improperly formatted line, missing winning numbers: {}", line).as_str())
            .trim()
            .split_ascii_whitespace()
            .map(|s| s.trim().parse::<i32>().expect(&format!("bad int error num={}", s)))
            .collect::<HashSet<_>>();
        let scratched = nums_line.next().expect(format!("Improperly formatted line, missing scratched numbers: {}", line).as_str())
            .trim()
            .replace("  ", " ")
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().expect(&format!("bad int error num={}", s)))
            .collect::<HashSet<_>>();
        let found = &winning & &scratched;
        return (game_id - 1, len!(found));
    }).collect::<Vec<_>>();

    let mut queue = lines_parsed.clone();
    while len!(queue) > 0 {
        let line = queue.pop().unwrap();
        total_count += 1;
        if line.1 > 0 {
            queue.extend_from_slice(&lines_parsed[line.0+1..=line.0+line.1]);
        }
    }
    return total_count;
}

