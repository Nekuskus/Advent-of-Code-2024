use setup_utils::{read_lines, len};
use std::path::Path;

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/03-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 4361 {
            Ok(())
        } else {
            Err(format!("03: Bad result for Part 1 example, expected 4361 got {}", result))
        }
    }

    /*#[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/01-2-example.txt"));
        let result = crate::part2(&lines);
        if result == 281 {
            Ok(())
        } else {
            Err(format!("03: Bad result for Part 2 example, expected 281 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/01-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);
        match (result1, result2) {
            (54159, 53866) => Ok(()),
            (_, 53866) => Err(format!("03: Bad result for Part 1, expected 54159 got {}", result1)),
            (54159, _) => Err(format!("03: Bad result for Part 2, expected 53866 got {}", result2)),
            (_, _) => Err(format!("03: Bad result for Part 1 & 2, expected (54159, 53866) got ({}, {})", result1, result2))
        }
    }*/
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/03-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/03-1-example.txt"));
    //let lines2 = read_lines(Path::new("./inputs/03-2-example.txt"));
    let a = vec![1, 2, 3, 4];
    println!("{}", len!(a));

    println!("03-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));
    
    println!("03-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
    
    
    //println!("03-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
    
}

fn part1(lines: &Vec::<String>) -> i32 {
    let sum_of_nums = 0;
    let cur_num = "";
    let cur_is_valid = false;
    for y in 0..lines.len() {
        for x in 0..len!(lines) {

        }
    }
    return sum_of_nums;
}