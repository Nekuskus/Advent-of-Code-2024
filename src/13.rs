use setup_utils::*;
use std::path::Path;
use debug_print::{debug_print as debug, debug_println as debugln};

// Symbols to replace: 13 405 TEST2 SOLVE1 SOLVE2


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/13-example.txt"));
        let result = crate::part1(&lines);
        if result == 405 {
            Ok(())
        } else {
            Err(format!("13: Bad result for Part 1 example, expected 405 got {}", result))
        }
    }
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/13-2-example.txt"));
        let result = crate::part2(&lines);
        if result == TEST2 {
            Ok(())
        } else {
            Err(format!("13: Bad result for Part 2 example, expected TEST2 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/13-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);
        
        if result1 == SOLVE1 {
            Ok(())
        } else {
            Err(format!("13: Bad result for Part 1, expected SOLVE1 got {}", result1))
        }
        /*
        match (result1, result2) {
            (SOLVE1, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("13: Bad result for Part 1, expected SOLVE1 got {}", result1)),
            (SOLVE1, _) => Err(format!("13: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("13: Bad result for Part 1 & 2, expected (SOLVE1, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
    */
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/13-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/13-example.txt"));
    //let lines2 = read_lines(Path::new("./inputs/13-2-example.txt"));

    println!("13-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));
    
    println!("13-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
    
    
    //println!("13-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
    
}


fn part1(lines: &Vec<String>) -> i32 {
    let sections = lines.split(|line| line == "").map(|arr| arr.iter().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<_>>>()).collect::<Vec<Vec<Vec<char>>>>();
    let mut cumsum = 0;

    let mut i = 0;
    for section in sections {
        let mut found_mirror = false;
        for y in 0..section.len() - 1 {
            if section[y] == section[y + 1] {
                let diff_from_line = match () {
                    _ if y + 1 > section.len() / 2 => section.len() - (y + 2),
                    _ => y// if y <= section.len() / 2
                };

                if (1..=diff_from_line).into_iter().all(|offset| {
                    section[y-offset] == section[y+1+offset]
                }) {
                    found_mirror = true;
                    debugln!("Section {i}, found at y: {}|{}", y, y+1);
                    debugln!("diff: {diff_from_line}");
                    cumsum += (y + 1) * 100;
                    break;
                }
            }
        }
        if !found_mirror {
            for x in 0..section[0].len() - 1 {
                if section.iter().all(|vec_char| vec_char[x] == vec_char[x+1]) {
                    let diff_from_line = match () {
                        _ if x + 1 > section[0].len() / 2 => section[0].len() - (x + 2),
                        _ => x// if y <= section.len() / 2
                    };
                    debugln!("diff_before_check: {diff_from_line}");
                    for line in &section {
                        debugln!("{}", line.iter().collect::<String>());
                    }
                    if (1..=diff_from_line).into_iter().all(|offset| {
                        debugln!("Checking offset {offset}, {} == {} for x: {}|{}", x-offset, x+1+offset, x, x+1);
                        for line in &section {
                            debugln!("{} == {}", line[x-offset], line[x+1+offset]);
                        }
                        section.iter().all(|vec_char| vec_char[x-offset] == vec_char[x+1+offset])
                    }) {
                        found_mirror = true;
                        debugln!("Section {i}, found at x: {}|{}", x+1, x+2);
                        debugln!("diff: {diff_from_line}");
                        cumsum += (x + 1);
                        break;
                    }
                }
            }
        }
        assert_eq!(found_mirror, true);
        i += 1;
    }

    cumsum as i32
}
/*
fn part2(lines: &Vec<String>) -> i32 {

}
*/