use utils::*;

// Symbols to replace: 04 18 9 2521 1912

#[cfg(test)]
mod tests {
    use utils::get_input;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = get_input!("04-example.txt");
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

    #[test]
    fn part2() -> Result<(), String> {
        let lines = get_input!("04-example.txt");
        let result = crate::part2(&lines);
        if result == 9 {
            Ok(())
        } else {
            Err(format!(
                "04: Bad result for Part 2 example, expected 9 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = get_input!("04-full.txt");
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (2521, 1912) => Ok(()),
            (_, 1912) => Err(format!(
                "04: Bad result for Part 1, expected 2521 got {}",
                result1
            )),
            (2521, _) => Err(format!(
                "04: Bad result for Part 2, expected 1912 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "04: Bad result for Part 1 & 2, expected (2521, 1912) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = get_input!("04-full.txt");
    let lines1 = get_input!("04-example.txt");

    println!("04-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("04-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
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

    iters
}

fn part1(lines: &Vec<String>) -> usize {
    let matrix: Vec<_> = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let needle = ['M', 'A', 'S'];
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

fn part2(lines: &Vec<String>) -> usize {
    let matrix: Vec<_> = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let needle1 = "MAS";
    let needle2 = "SAM";

    let mut count = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == 'A'
                && x > 0 && x < line.len() - 1 && y > 0 && y < matrix.len() - 1 {
                    let strings: [String; 2] = [
                        [matrix[y - 1][x - 1], 'A', matrix[y + 1][x + 1]]
                            .iter()
                            .collect(),
                        [matrix[y - 1][x + 1], 'A', matrix[y + 1][x - 1]]
                            .iter()
                            .collect(),
                    ];

                    if strings.iter().all(|s| s == needle1 || s == needle2) {
                        count += 1;
                    }
                }
        }
    }

    count
}
