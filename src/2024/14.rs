use itertools::Itertools;
use utils::*;

// Symbols to replace: 14 12 TEST2 214109808 SOLVE2

// #[cfg(test)]
// mod tests {
//     use utils::get_input;
//     use std::path::Path;

//     #[test]
//     fn part1() -> Result<(), String> {
//         let lines = get_input!("14-example.txt");
//         let result = crate::part1(&lines, 11, 7);
//         if result == 12 {
//             Ok(())
//         } else {
//             Err(format!(
//                 "14: Bad result for Part 1 example, expected 12 got {}",
//                 result
//             ))
//         }
//     }

//     #[test]
//     fn full() -> Result<(), String> {
//         let lines = get_input!("14-full.txt");
//         let result1 = crate::part1(&lines, 101, 103);
//         //let result2 = crate::part2(&lines);

//         if result1 == 214109808 {
//             Ok(())
//         } else {
//             Err(format!(
//                 "14: Bad result for Part 1, expected 214109808 got {}",
//                 result1
//             ))
//         }
//         /*
//         match (result1, result2) {
//             (214109808, SOLVE2) => Ok(()),
//             (_, SOLVE2) => Err(format!(
//                 "14: Bad result for Part 1, expected 214109808 got {}",
//                 result1
//             )),
//             (214109808, _) => Err(format!(
//                 "14: Bad result for Part 2, expected SOLVE2 got {}",
//                 result2
//             )),
//             (_, _) => Err(format!(
//                 "14: Bad result for Part 1 & 2, expected (214109808, SOLVE2) got ({}, {})",
//                 result1, result2
//             )),
//         }*/
//     }
// }

fn main() {
    let linesfull = get_input!("14-full.txt");
    let lines1 = get_input!("14-example.txt");

    println!("14-full.txt");
    println!("{}", part1(&linesfull, 101, 103));
    println!("{}\n", part2(&linesfull, 101, 103));

    println!("14-example.txt");
    println!("{}", part1(&lines1, 11, 7));
    println!("{}\n", part2(&lines1, 11, 7));
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    p: PointI,
    v: PointI,
}

fn part1(lines: &Vec<String>, xlen: usize, ylen: usize) -> usize {
    let re = regex::Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    let mut robots = lines
        .iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Robot {
                p: PointI::new(
                    caps.name("px").unwrap().as_str().parse::<isize>().unwrap(),
                    caps.name("py").unwrap().as_str().parse::<isize>().unwrap(),
                ),
                v: PointI::new(
                    caps.name("vx").unwrap().as_str().parse::<isize>().unwrap(),
                    caps.name("vy").unwrap().as_str().parse::<isize>().unwrap(),
                ),
            }
        })
        .collect_vec();

    // robots.iter_mut().for_each(|r| {
    //     r.p.x = (r.p.x + r.v.x * 100).rem_euclid(xlen as isize);
    //     r.p.y = (r.p.y + r.v.y * 100).rem_euclid(ylen as isize);
    // });

    robots.iter_mut().for_each(|r| {
        r.p.x = (r.p.x + r.v.x * 7687).rem_euclid(xlen as isize);
        r.p.y = (r.p.y + r.v.y * 7687).rem_euclid(ylen as isize);
    });

    for y in 0..ylen {
        for x in 0..xlen {
            if y == (ylen - 1) / 2 || x == (xlen - 1) / 2 {
                print!(" ");
                continue;
            }

            let count = robots
                .iter()
                .filter(|r| r.p.x == x as isize && r.p.y == y as isize)
                .count();
            if count > 0 {
                print!("{count}");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    robots
        .iter()
        .filter(|r| r.p.x < (xlen as isize - 1) / 2 && r.p.y < (ylen as isize - 1) / 2)
        .count()
        * robots
            .iter()
            .filter(|r| r.p.x > (xlen as isize - 1) / 2 && r.p.y < (ylen as isize - 1) / 2)
            .count()
        * robots
            .iter()
            .filter(|r| r.p.x < (xlen as isize - 1) / 2 && r.p.y > (ylen as isize - 1) / 2)
            .count()
        * robots
            .iter()
            .filter(|r| r.p.x > (xlen as isize - 1) / 2 && r.p.y > (ylen as isize - 1) / 2)
            .count()
}

fn part2(lines: &Vec<String>, xlen: usize, ylen: usize) -> usize {
    let re = regex::Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    let mut robots = lines
        .iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Robot {
                p: PointI::new(
                    caps.name("px").unwrap().as_str().parse::<isize>().unwrap(),
                    caps.name("py").unwrap().as_str().parse::<isize>().unwrap(),
                ),
                v: PointI::new(
                    caps.name("vx").unwrap().as_str().parse::<isize>().unwrap(),
                    caps.name("vy").unwrap().as_str().parse::<isize>().unwrap(),
                ),
            }
        })
        .collect_vec();
    (0..u16::MAX)
        .map(|idx| {
            robots.iter_mut().for_each(|r| {
                r.p.x = (r.p.x + r.v.x).rem_euclid(xlen as isize);
                r.p.y = (r.p.y + r.v.y).rem_euclid(ylen as isize);
            });

            // for y in 0..ylen {
            //     for x in 0..xlen {
            //         if y == (ylen - 1) / 2 && x == (xlen - 1) / 2 {
            //             print!(" ");
            //             continue;
            //         }

            //         let count = robots
            //             .iter()
            //             .filter(|r| r.p.x == x as isize && r.p.y == y as isize)
            //             .count();
            //         if count > 0 {
            //             print!("{count}");
            //         } else {
            //             print!(".");
            //         }
            //     }
            //     println!();
            // }
            // println!();
            // sleep(Duration::from_millis(100));

            (
                idx,
                robots
                    .iter()
                    .filter(|r| r.p.x < (xlen as isize - 1) / 2 && r.p.y < (ylen as isize - 1) / 2)
                    .count()
                    * robots
                        .iter()
                        .filter(|r| {
                            r.p.x > (xlen as isize - 1) / 2 && r.p.y < (ylen as isize - 1) / 2
                        })
                        .count()
                    * robots
                        .iter()
                        .filter(|r| {
                            r.p.x < (xlen as isize - 1) / 2 && r.p.y > (ylen as isize - 1) / 2
                        })
                        .count()
                    * robots
                        .iter()
                        .filter(|r| {
                            r.p.x > (xlen as isize - 1) / 2 && r.p.y > (ylen as isize - 1) / 2
                        })
                        .count(),
            )
        })
        .min_by_key(|&(_idx, cost)| cost)
        .unwrap()
        .0 as usize
        + 1
}
