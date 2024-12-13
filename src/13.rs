use itertools::Itertools;
use regex::Regex;
use setup_utils::*;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    path::Path,
};

// Symbols to replace: 13 480 TEST2 35082 SOLVE2

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
        //let result2 = crate::part2(&lines);

        if result1 == 35082 {
            Ok(())
        } else {
            Err(format!(
                "13: Bad result for Part 1, expected 35082 got {}",
                result1
            ))
        }
        /*
        match (result1, result2) {
            (35082, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!(
                "13: Bad result for Part 1, expected 35082 got {}",
                result1
            )),
            (35082, _) => Err(format!(
                "13: Bad result for Part 2, expected SOLVE2 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "13: Bad result for Part 1 & 2, expected (35082, SOLVE2) got ({}, {})",
                result1, result2
            )),
        }*/
    }
}

struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/13-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/13-1-example.txt"));
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
fn optimise_machine(
    cache: &mut HashMap<(Point, u32), Option<u32>>,
    m: &Machine,
    cur: Point,
    spent: u32,
) -> Option<u32> {
    let cost_a = 3;
    let cost_b = 1;

    if cache.contains_key(&(cur, spent)) {
        return cache.get(&(cur, spent)).unwrap().clone();
    }

    if cur > m.prize {
        return None;
    }

    if cur == m.prize {
        return Some(spent);
    }

    // if counter >= 100 {
    //     return None;
    // }

    let (ret_a, ret_b) = (
        optimise_machine(
            cache,
            m,
            Point::new(cur.x + m.a.x, cur.y + m.a.y),
            spent + cost_a,
            // counter + 1,
        ),
        optimise_machine(
            cache,
            m,
            Point::new(cur.x + m.b.x, cur.y + m.b.y),
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

fn part1(lines: &Vec<String>) -> u32 {
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
        .map(|m| optimise_machine(&mut HashMap::new(), &m, Point::new(0, 0), 0))
        .filter_map(|opt| opt)
        .sum()
}
/*
fn part2(lines: &Vec<String>) -> i32 {

}
*/
