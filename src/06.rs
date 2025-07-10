use setup_utils::*;
use std::path::Path;

// Symbols to replace: 06 288 TEST2 SOLVE1 SOLVE2


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/06-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 288 {
            Ok(())
        } else {
            Err(format!("06: Bad result for Part 1 example, expected 288 got {}", result))
        }
    }
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/06-2-example.txt"));
        let result = crate::part2(&lines);
        if result == TEST2 {
            Ok(())
        } else {
            Err(format!("06: Bad result for Part 2 example, expected TEST2 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/06-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);
        
        if result1 == 288 {
            Ok(())
        } else {
            Err(format!("06: Bad result for Part 1, expected 288 got {}", result1))
        }
        /*
        match (result1, result2) {
            (SOLVE1, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("06: Bad result for Part 1, expected SOLVE1 got {}", result1)),
            (SOLVE1, _) => Err(format!("06: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("06: Bad result for Part 1 & 2, expected (SOLVE1, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
    */
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/06-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/06-1-example.txt"));
    //let lines2 = read_lines(Path::new("./inputs/06-2-example.txt"));

    println!("06-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));
    
    println!("06-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
    
    
    //println!("06-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
    
}

#[inline(always)]
fn delay_to_mm(delay: &i32, length: &i32) -> i32 {
    let mut i = delay.to_owned();
    let mut val = 0;
    while i < length.to_owned() {
        val += delay;
        i += 1;
    }
    return val; 
}

fn part1(lines: &Vec::<String>) -> i32 {
    let mut power = 1;
    let mut counts = Vec::new();

    let times = lines[0].split_ascii_whitespace().collect::<Vec<_>>()[1..].iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let distances = lines[1].split_ascii_whitespace().collect::<Vec<_>>()[1..].iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    
    let zip = times.iter().zip(distances.iter());
    
    for (time, distance) in zip {
        let mut vec = Vec::with_capacity(*time as usize);

        for delay in 1..*time {
            vec.push(delay_to_mm(&delay, time))
        }

        vec = vec.iter().filter(|res| *res > distance).copied().collect::<Vec<_>>();
        counts.push(len!(vec) as i32);
    }

    counts.iter().for_each(|count| {
        power *= count;
    });

    return power;
}
/*
fn part2(lines: &Vec::<String>) -> i32 {

}
*/