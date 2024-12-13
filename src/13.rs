use itertools::Itertools;
use regex::Regex;
use setup_utils::*;
use std::{collections::HashMap, path::Path};

// Symbols to replace: 13 480 TEST2 35082 82570698600470

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/13-example.txt"));
        let result = crate::part1(&lines);
        if result == 480 {
            Ok(())
        } else {
            Err(format!(
                "13: Bad result for Part 1 example, expected 480 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/13-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (35082, 82570698600470) => Ok(()),
            (_, 82570698600470) => Err(format!(
                "13: Bad result for Part 1, expected 35082 got {}",
                result1
            )),
            (35082, _) => Err(format!(
                "13: Bad result for Part 2, expected 82570698600470 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "13: Bad result for Part 1 & 2, expected (35082, 82570698600470) got ({}, {})",
                result1, result2
            )),
        }
    }
}

struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/13-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/13-example.txt"));

    println!("13-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("13-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

#[allow(unused)]
fn optimise_machine(
    cache: &mut HashMap<(Point, u32), Option<u32>>,
    mac: &Machine,
    cur: Point,
    spent: u32,
) -> Option<u32> {
    let cost_a = 3;
    let cost_b = 1;

    if cache.contains_key(&(cur, spent)) {
        return cache.get(&(cur, spent)).unwrap().clone();
    }

    if cur > mac.prize {
        return None;
    }

    if cur == mac.prize {
        return Some(spent);
    }

    // if counter >= 100 {
    //     return None;
    // }

    let (ret_a, ret_b) = (
        optimise_machine(
            cache,
            mac,
            Point::new(cur.x + mac.a.x, cur.y + mac.a.y),
            spent + cost_a,
            // counter + 1,
        ),
        optimise_machine(
            cache,
            mac,
            Point::new(cur.x + mac.b.x, cur.y + mac.b.y),
            spent + cost_b,
            // counter + 1,
        ),
    );

    let ret = match (ret_a, ret_b) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (Some(a), None) => Some(a),
        (None, Some(b)) => Some(b),
        (None, None) => None,
    };

    cache.insert((cur, spent), ret);

    ret
}

fn part1(lines: &Vec<String>) -> u64 {
    let re_button = Regex::new(r"X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let re_prize = Regex::new(r"X=(?<x>\d+), Y=(?<y>\d+)").unwrap();

    lines
        .iter()
        .filter(|l| l.len() != 0)
        .chunks(3)
        .into_iter()
        .map(|mut c| {
            let l1 = c.next().unwrap();
            let l2 = c.next().unwrap();
            let l3 = c.next().unwrap();

            let caps_a = re_button.captures(l1).unwrap();
            let caps_b = re_button.captures(l2).unwrap();
            let caps_prize = re_prize.captures(l3).unwrap();

            Machine {
                a: Point::new(
                    caps_a.name("x").unwrap().as_str().parse().unwrap(),
                    caps_a.name("y").unwrap().as_str().parse().unwrap(),
                ),
                b: Point::new(
                    caps_b.name("x").unwrap().as_str().parse().unwrap(),
                    caps_b.name("y").unwrap().as_str().parse().unwrap(),
                ),
                prize: Point::new(
                    caps_prize.name("x").unwrap().as_str().parse().unwrap(),
                    caps_prize.name("y").unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        // .map(|m| optimise_machine(&mut HashMap::new(), &m, Point::new(0, 0), 0))
        .map(|m| solve_matrix(machine_to_matrix(&m)))
        .filter_map(|opt| opt)
        .map(|(a, b)| {
            if a >= 0.0 && a.round() == a && b >= 0.0 && b.round() == b {
                (a * 3.0 + b) as u64
            } else {
                0
            }
        })
        .sum()
}

fn machine_to_matrix(mac: &Machine) -> [[f64; 3]; 2] {
    [
        [mac.a.x as f64, mac.b.x as f64, mac.prize.x as f64],
        [mac.a.y as f64, mac.b.y as f64, mac.prize.y as f64],
    ]
}

fn solve_matrix(m: [[f64; 3]; 2]) -> Option<(f64, f64)> {
    let a1 = m[0][0];
    let b1 = m[0][1];
    let c1 = m[0][2];

    let a2 = m[1][0];
    let b2 = m[1][1];
    let c2 = m[1][2];

    let num1 = c1 * b2 - b1 * c2;
    let denom1 = a1 * b2 - a2 * b1;

    if denom1 == 0.0 {
        return None;
    }

    let x = num1 / denom1;

    let num2 = c1 - a1 * x;
    let denom2 = b1;

    if denom2 == 0.0 {
        return None;
    }

    let y = num2 / denom2;

    return Some((x, y));
}

fn part2(lines: &Vec<String>) -> u64 {
    let re_button = Regex::new(r"X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let re_prize = Regex::new(r"X=(?<x>\d+), Y=(?<y>\d+)").unwrap();

    lines
        .iter()
        .filter(|l| l.len() != 0)
        .chunks(3)
        .into_iter()
        .map(|mut c| {
            let l1 = c.next().unwrap();
            let l2 = c.next().unwrap();
            let l3 = c.next().unwrap();

            let caps_a = re_button.captures(l1).unwrap();
            let caps_b = re_button.captures(l2).unwrap();
            let caps_prize = re_prize.captures(l3).unwrap();

            Machine {
                a: Point::new(
                    caps_a.name("x").unwrap().as_str().parse().unwrap(),
                    caps_a.name("y").unwrap().as_str().parse().unwrap(),
                ),
                b: Point::new(
                    caps_b.name("x").unwrap().as_str().parse().unwrap(),
                    caps_b.name("y").unwrap().as_str().parse().unwrap(),
                ),
                prize: Point::new(
                    10000000000000usize
                        + caps_prize
                            .name("x")
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap(),
                    10000000000000usize
                        + caps_prize
                            .name("y")
                            .unwrap()
                            .as_str()
                            .parse::<usize>()
                            .unwrap(),
                ),
            }
        })
        .map(|m| solve_matrix(machine_to_matrix(&m)))
        .filter_map(|opt| opt)
        .map(|(a, b)| {
            if a >= 0.0 && a.round() == a && b >= 0.0 && b.round() == b {
                (a * 3.0 + b) as u64
            } else {
                0
            }
        })
        .sum::<u64>()
}
