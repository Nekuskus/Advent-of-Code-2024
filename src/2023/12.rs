use setup_utils::*;
use std::path::Path;

// Symbols to replace: 12 21 525152 7922 SOLVE2


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/12-example.txt"));
        let result = crate::part1(&lines);
        if result == 21 {
            Ok(())
        } else {
            Err(format!("12: Bad result for Part 1 example, expected 21 got {}", result))
        }
    }
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/12-example.txt"));
        let result = crate::part2(&lines);
        if result == 525152 {
            Ok(())
        } else {
            Err(format!("12: Bad result for Part 2 example, expected 525152 got {}", result))
        }
    }
    */
    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/12-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);
        
        if result1 == 7922 {
            Ok(())
        } else {
            Err(format!("12: Bad result for Part 1, expected 7922 got {}", result1))
        }
        /*
        match (result1, result2) {
            (7922, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("12: Bad result for Part 1, expected 7922 got {}", result1)),
            (7922, _) => Err(format!("12: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("12: Bad result for Part 1 & 2, expected (7922, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
    
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/12-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/12-1-example.txt"));
    //let lines2 = read_lines(Path::new("./inputs/12-2-example.txt"));

    println!("12-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));
    
    println!("12-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
}

fn get_next_permutation(line: &String, pat: &String) -> Option<String> {
    if pat.chars().zip(line.chars()).all(|item| { // Already iterated through all possibilities
        if item.0 == item.1 {
            return true
        } else if item.0 == '?' && item.1 == '#' {
            return true
        } else {
            return false;
        }
    }) {
        return None;
    }

    let marked_indices = pat.chars().enumerate().filter(|(idx, c)| c == &'?').map(|(idx, c)| idx).collect::<Vec<usize>>();

    let mut first_broken = usize::MAX;
    let mut first_working = usize::MAX;
    
    let mut line_chars = line.chars().collect::<Vec<_>>();
    //println!("{marked_indices:?}");
    for idx in &marked_indices {
        if first_working == usize::MAX && line_chars[*idx] == '.' {
            first_working = *idx;
        } else if first_broken == usize::MAX && line_chars[*idx] == '#' {
            first_broken = *idx;
        }

        if first_working != usize::MAX && first_broken != usize::MAX {
            break;
        }
    }

    if first_broken == usize::MAX || first_working < first_broken {
        line_chars[first_working] = '#';
    } else if first_working > first_broken {
        for x in marked_indices.iter().filter(|idx| idx < &&first_working) {
            line_chars[*x] = '.';
        }
        line_chars[first_working] = '#';
    }

    return Some(line_chars.iter().collect());
}

fn check_validity(line: &String, criteria: &Vec<u128>) -> bool {
    let split = line.replace(".", " ").split_ascii_whitespace().map(|s| s.len() as u128).collect::<Vec<u128>>();
    return &split == criteria;
}

fn part1(lines: &Vec<String>) -> u128 {
    let parsed = lines.iter().map(|s| {
        let split = s.split(" ").collect::<Vec<_>>();
        let criteria: Vec<u128> = split[1].split(',').map(|s| s.parse().unwrap()).collect();
        return (split[0], criteria);
    }).collect::<Vec<_>>();

    let mut count = 0;
    for (i, (line, criteria)) in parsed.iter().enumerate() {
        //println!("i = {i}, {criteria:?}");
        let pat = line.to_string();
        //println!("{pat}");
        let mut next = Some(pat.replace("?", "."));
        while next.is_some() {
            let unwrapped = next.unwrap();
            if check_validity(&unwrapped, criteria) {
                count += 1;
                //println!("{}", unwrapped);
            }
            next = get_next_permutation(&unwrapped, &pat);
        }
    }
    return count;
}

pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

fn part2(lines: &Vec<String>) -> u128 {
    let parsed = lines.iter().map(|s| {
        let split = s.split(" ").collect::<Vec<_>>();
        let criteria: Vec<u128> = split[1].split(',').map(|s| s.parse().unwrap()).collect();
        //return (split[0], criteria);
        
        return (vec![split[0]].repeat(5).join("?"), criteria.repeat(5));
    }).collect::<Vec<_>>();

    let mut count = 0;
    for (i, (line, criteria)) in parsed.iter().enumerate() {
        println!("i = {i}, {criteria:?}");
        let pat = line.to_string();
        //println!("{pat}");
        let mut next = Some(pat.replace("?", "."));
        let mut count_per_i: u128 = 0;
        while next.is_some() {
            let unwrapped = next.unwrap();
            if check_validity(&unwrapped, &criteria) {
                count_per_i += 1;
            }
            println!("{}", unwrapped);
            next = get_next_permutation(&unwrapped, &pat);
        }
        //count_per_i = 5_u128.pow((count_per_i as u32) - 1);
        println!("total = {count_per_i}");
        count += count_per_i;
    }
    return count;
}
