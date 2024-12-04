use debug_print::{debug_print as debug, debug_println as debugln};
use setup_utils::*;
use std::path::Path;

// Symbols to replace: 04 18 TEST2 SOLVE1 SOLVE2

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/04-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 18 {
            Ok(())
        } else {
            Err(format!(
                "04: Bad result for Part 1 example, expected 18 got {}",
                result
            ))
        }
    }
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/04-2-example.txt"));
        let result = crate::part2(&lines);
        if result == TEST2 {
            Ok(())
        } else {
            Err(format!("04: Bad result for Part 2 example, expected TEST2 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/04-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);

        if result1 == SOLVE1 {
            Ok(())
        } else {
            Err(format!("04: Bad result for Part 1, expected SOLVE1 got {}", result1))
        }
        /*
        match (result1, result2) {
            (SOLVE1, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("04: Bad result for Part 1, expected SOLVE1 got {}", result1)),
            (SOLVE1, _) => Err(format!("04: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("04: Bad result for Part 1 & 2, expected (SOLVE1, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
    */
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/04-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/04-1-example.txt"));
    //let lines2 = read_lines(Path::new("./inputs/04-2-example.txt"));

    println!("04-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));

    println!("04-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));

    //println!("04-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
}

fn gen_iter(
    matrix: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    max_diff: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut iters = vec![vec![]; 8];
    let ylen = matrix[0].len();
    let xlen = matrix.len();

    for diff in 1..(max_diff + 1) {
        if x >= diff {
            if y >= diff {
                iters[0].push((x - diff, y - diff));
            }
            iters[1].push((x - diff, y));
            if y < ylen - diff {
                iters[2].push((x - diff, y + diff));
            }
        }

        if y >= diff {
            iters[3].push((x, y - diff));
        }

        if y < ylen - diff {
            iters[4].push((x, y + diff));
        }

        if x < xlen - diff {
            if y >= diff {
                iters[5].push((x + diff, y - diff));
            }
            iters[6].push((x + diff, y));
            if y < ylen - diff {
                iters[7].push((x + diff, y + diff));
            }
        }
    }
    println!("{x},{y}: {iters:?}");
    iters
}

fn part1(lines: &Vec<String>) -> usize {
    let matrix: Vec<_> = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let needle = vec!['M', 'A', 'S'];
    let mut count = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            let iters = gen_iter(&matrix, x, y, 3);
            if c == 'X' {
                count += iters
                    .iter()
                    .filter(|v| {
                        v.iter()
                            .map(|&(x_find, y_find)| matrix[y_find][x_find])
                            .eq(needle.iter().copied())
                    })
                    .count();
            }
        }
    }

    count
}

/*
fn part2(lines: &Vec<String>) -> i32 {

}
*/
