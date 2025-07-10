use setup_utils::read_lines;
use std::path::Path;

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/01-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 142 {
            Ok(())
        } else {
            Err(format!("01: Bad result for Part 1 example, expected 142 got {}", result))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/01-2-example.txt"));
        let result = crate::part2(&lines);
        if result == 281 {
            Ok(())
        } else {
            Err(format!("01: Bad result for Part 2 example, expected 281 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/01-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);
        match (result1, result2) {
            (54159, 53866) => Ok(()),
            (_, 53866) => Err(format!("02: Bad result for Part 1, expected 54159 got {}", result1)),
            (54159, _) => Err(format!("02: Bad result for Part 2, expected 53866 got {}", result2)),
            (_, _) => Err(format!("02: Bad result for Part 1 & 2, expected (54159, 53866) got ({}, {})", result1, result2))
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/01-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/01-1-example.txt"));
    let lines2 = read_lines(Path::new("./inputs/01-2-example.txt"));

    println!("01-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));
    
    println!("01-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
    
    
    println!("01-2-example.txt");
    println!("{}", part1(&lines2));
    println!("{}", part2(&lines2));
    
}

fn part1(lines: &Vec::<String>) -> i32 {
    let mut sums = Vec::new();
    for line in lines {
        let mut first = String::new();
        let mut last = String::new();
        for c in line.chars() {
            if !c.is_numeric() {
                continue;
            }
            if first == "" {
                first = c.to_string();
            }
            last = c.to_string();
        }
        let sum = first + &last;
        sums.push(sum);
    }
    let mut nums = Vec::new();
    for sum in sums {
        if sum != "" {
            nums.push(sum.parse::<i32>().unwrap());
        }
    }
    let sum = nums.iter().sum::<i32>();
    
    return sum
}

// one, two, three, four, five, six, seven, eight, nine
fn part2(lines: &Vec::<String>) -> i32 {
    let mut sums = Vec::new();
    for line in lines {
        let mut first = String::new();
        let mut last = String::new();
        let chars = line.chars().collect::<Vec<char>>();
        for i in 0..line.len() {
            if chars[i].is_numeric() {
                if first == "" {
                    first = chars[i].to_string();
                }
                last = chars[i].to_string();
            } else {
                let mut slice = chars[i..i].iter();
                if i+5 <= chars.len() {
                    slice = chars[i..i+5].iter();
                } else if i+4 <= chars.len() {
                    slice = chars[i..i+4].iter();
                } else if i+3 <= chars.len() {
                    slice = chars[i..i+3].iter();
                }
                let potential_num = String::from_iter(slice);
                let found_num = match potential_num.as_str() {
                    _ if potential_num.starts_with("one") => "1",
                    _ if potential_num.starts_with("two") => "2",
                    _ if potential_num.starts_with("three") => "3",
                    _ if potential_num.starts_with("four") => "4",
                    _ if potential_num.starts_with("five") => "5",
                    _ if potential_num.starts_with("six") => "6",
                    _ if potential_num.starts_with("seven") => "7",
                    _ if potential_num.starts_with("eight") => "8",
                    _ if potential_num.starts_with("nine") => "9",
                    _ => "-1"
                }.to_owned();
                if first == "" && found_num != "-1" {
                    //println!("{}: {}", potential_num, found_num);
                    first = found_num.clone();
                }
                if found_num != "-1" {
                    last = found_num;
                }
            }
        }
        //println!("{}: {} {}", line, first, last);
        let sum = first + &last;
        sums.push(sum);
    }
    let mut nums = Vec::new();
    for sum in sums {
        nums.push(sum.parse::<i32>().unwrap());
    }
    let sum = nums.iter().sum::<i32>();
    
    return sum
}
