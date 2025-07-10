use setup_utils::*;
use std::path::Path;

// Symbols to replace: 06 288 71503 2449062 33149631


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/06-example.txt"));
        let result = crate::part1(&lines);
        if result == 288 {
            Ok(())
        } else {
            Err(format!("06: Bad result for Part 1 example, expected 288 got {}", result))
        }
    }
    
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/06-example.txt"));
        let result = crate::part2(&lines);
        if result == 71503 {
            Ok(())
        } else {
            Err(format!("06: Bad result for Part 2 example, expected 71503 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/06-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (2449062, 33149631) => Ok(()),
            (_, 33149631) => Err(format!("06: Bad result for Part 1, expected 2449062 got {}", result1)),
            (2449062, _) => Err(format!("06: Bad result for Part 2, expected 33149631 got {}", result2)),
            (_, _) => Err(format!("06: Bad result for Part 1 & 2, expected (2449062, 33149631) got ({}, {})", result1, result2))
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
    println!("{}\n", part2(&lines1));
}

fn part1(lines: &Vec::<String>) -> u128 {
    let power = 1;
    let mut counts = Vec::new();

    let times = lines[0].split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u128>().unwrap());
    let distances = lines[1].split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u128>().unwrap());
    
    let zip = times.zip(distances);
    
    for (time, distance) in zip {
        let mut vec = Vec::with_capacity(time as usize);

        for delay in 1..time {
            let val = delay * (time - delay);
            vec.push(val)
        }

        vec = vec.iter().filter(|res| **res > distance).copied().collect::<Vec<_>>();
        counts.push(len!(vec) as u128);
    }

    counts.iter().fold(power, |cur_power, count| {
        cur_power * count
    })
}

fn part2(lines: &Vec::<String>) -> u128 {
    let mut newlines = lines.clone();
    newlines[0] = "Time: ".to_owned() + &newlines[0].split_ascii_whitespace()
        .collect::<Vec<_>>()[1..].to_vec() // Somehow faster than using .skip(1) on the iter
        .join("");
    newlines[1] = "Distance: ".to_owned() + &newlines[1].split_ascii_whitespace()
        .collect::<Vec<_>>()[1..].to_vec()
        .join("");
    println!("{}", newlines[0]);
    println!("{}", newlines[1]);
    return part1(&newlines);
}