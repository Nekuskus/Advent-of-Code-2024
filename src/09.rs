use setup_utils::*;
use std::path::Path;

// Symbols to replace: 09 114 TEST2 2043183816 SOLVE2


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/09-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 114 {
            Ok(())
        } else {
            Err(format!("09: Bad result for Part 1 example, expected 114 got {}", result))
        }
    }
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/09-2-example.txt"));
        let result = crate::part2(&lines);
        if result == TEST2 {
            Ok(())
        } else {
            Err(format!("09: Bad result for Part 2 example, expected TEST2 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/09-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);
        
        if result1 == 2043183816 {
            Ok(())
        } else {
            Err(format!("09: Bad result for Part 1, expected 2043183816 got {}", result1))
        }
        /*
        match (result1, result2) {
            (2043183816, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("09: Bad result for Part 1, expected 2043183816 got {}", result1)),
            (2043183816, _) => Err(format!("09: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("09: Bad result for Part 1 & 2, expected (2043183816, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
    */
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/09-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/09-1-example.txt"));
    //let lines2 = read_lines(Path::new("./inputs/09-2-example.txt"));

    println!("09-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));
    
    println!("09-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
    
    
    //println!("09-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
    
}


fn evaluate_history(nums: Vec<i32>) -> i32 {
    //println!("{nums:?}");
    if nums.iter().all(|n| n == &0) {
        return 0;
    }
    let mut diffs = Vec::new();
    for i in 0..len!(nums)-1 {
        diffs.push(nums[i+1] - nums[i]);
    }
    let extrapolated_diff = evaluate_history(diffs);
    let extrapolated = nums.last().unwrap() + extrapolated_diff;
    return extrapolated;
}

fn part1(lines: &Vec<String>) -> i32 {
    let total = lines.iter().map(|s| s.split(" ").map(|item| item.parse::<i32>().unwrap()).collect::<Vec<_>>()).map(evaluate_history).sum();
    return total;
}
/*
fn part2(lines: &Vec<String>) -> i32 {

}
*/