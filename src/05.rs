use setup_utils::*;
use std::path::Path;

// Symbols to replace: 05 35 TEST2 SOLVE1 SOLVE2


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 35 {
            Ok(())
        } else {
            Err(format!("05: Bad result for Part 1 example, expected 35 got {}", result))
        }
    }
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-2-example.txt"));
        let result = crate::part2(&lines);
        if result == TEST2 {
            Ok(())
        } else {
            Err(format!("05: Bad result for Part 2 example, expected TEST2 got {}", result))
        }
    }
    /*
    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);
        
        if resul1 == 35 {
            Ok(())
        } else {
            Err(format!("05: Bad result for Part 1, expected 35 got {}", result))
        }
        /*
        match (result1, result2) {
            (SOLVE1, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("05: Bad result for Part 1, expected SOLVE1 got {}", result1)),
            (SOLVE1, _) => Err(format!("05: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("05: Bad result for Part 1 & 2, expected (SOLVE1, SOLVE2) got ({}, {})", result1, result2))
        }
        */
    }*/
    */
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/05-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/05-1-example.txt"));
    let lines2 = read_lines(Path::new("./inputs/05-2-example.txt"));

    println!("05-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));
    
    println!("05-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
    
    
    //println!("05-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
    
}


fn part1(lines: &Vec::<String>) -> i32 {

}
/*
fn part2(lines: &Vec::<String>) -> i32 {

}
*/