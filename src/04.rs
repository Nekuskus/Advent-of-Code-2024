use setup_utils::{len, read_lines};
use std::path::Path;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/04-1-example.txt"));
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
/*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/03-example.txt"));
        let result = crate::part2(&lines);
        if result == 467835 {
            Ok(())
        } else {
            Err(format!(
                "03: Bad result for Part 2 example, expected 467835 got {}",
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
            (525181, 84289137) => Ok(()),
            (_, 84289137) => Err(format!(
                "03: Bad result for Part 1, expected 525181 got {}",
                result1
            )),
            (525181, _) => Err(format!(
                "03: Bad result for Part 2, expected 84289137 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "03: Bad result for Part 1 & 2, expected (525181, 84289137) got ({}, {})",
                result1, result2
            )),
        }
    }*/
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/04-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/04-1-example.txt"));

    println!("04-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));

    println!("04-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
}


fn part1(lines: &Vec<String>) -> i32 {
   let mut sum_of_points = 0;
   for line in lines {
       let nums_line = line.split(":").collect::<Vec<_>>()[1].trim().split('|').map(|s| s.trim()).collect::<Vec<_>>();
       let winning = nums_line[0].trim().replace("  ", " ").split(' ').map(|s| s.trim().parse::<i32>().expect("bad int error")).collect::<HashSet<_>>();
//       let deb = nums_line[1].trim().split(' ').map(|s| s.to_owned()).collect::<Vec<String>>();
//       println!("{:?}", deb);
       let scratched = nums_line[1].trim().replace("  ", " ").split(' ').map(|s| s.parse::<i32>().expect(&format!("bad int error num={}", s))).collect::<HashSet<_>>();
       let found: HashSet<i32> = winning.intersection(&scratched).map(|n| n.to_owned()).collect();
       if len!(found) > 0 {
           let score = 2_i32.pow(len!(found) as u32 - 1);
           sum_of_points += score;
           println!("a {:?}\nb {:?}\nc {:?} score {}", winning, scratched, found, score);
       }
   }
   return sum_of_points;
}

//fn part2(lines: &Vec<String>) -> i32 {
//}
