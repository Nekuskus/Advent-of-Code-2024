use itertools::Itertools;
use utils::*;
use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

// Symbols to replace: 08 14 9 390 1246

#[cfg(test)]
mod tests {
    use utils::{get_input, Point};

    #[test]
    fn are_collinear() {
        assert_eq!(
            crate::are_collinear(vec![
                &Point::new(1, 2),
                &Point::new(2, 3),
                &Point::new(3, 4)
            ]),
            true
        );

        assert_eq!(
            crate::are_collinear(vec![
                &Point::new(1, 2),
                &Point::new(1, 3),
                &Point::new(1, 4)
            ]),
            true
        );
        assert_eq!(
            crate::are_collinear(vec![
                &Point::new(1, 2),
                &Point::new(2, 4),
                &Point::new(3, 6)
            ]),
            true
        );
    }

    #[test]
    fn part1() -> Result<(), String> {
        let lines = get_input!("08-1-example.txt");
        let result = crate::part1(&lines, false);
        if result == 14 {
            Ok(())
        } else {
            Err(format!(
                "08: Bad result for Part 1 example, expected 14 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = get_input!("08-2-example.txt");
        let result = crate::part2(&lines);
        if result == 9 {
            Ok(())
        } else {
            Err(format!(
                "08: Bad result for Part 2 example, expected 9 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = get_input!("08-full.txt");
        let result1 = crate::part1(&lines, false);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (390, 1246) => Ok(()),
            (_, 1246) => Err(format!(
                "08: Bad result for Part 1, expected 390 got {}",
                result1
            )),
            (390, _) => Err(format!(
                "08: Bad result for Part 2, expected 1246 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "08: Bad result for Part 1 & 2, expected (390, 1246) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = get_input!("08-full.txt");
    let lines1 = get_input!("08-1-example.txt");
    let lines2 = get_input!("08-2-example.txt");

    println!("08-full.txt");
    println!("{}", part1(&linesfull, false));
    println!("{}\n", part2(&linesfull));

    println!("08-1-example.txt");
    println!("{}", part1(&lines1, false));
    println!("{}\n", part2(&lines1));

    println!("08-2-example.txt");
    println!("{}", part1(&lines2, false));
    println!("{}", part2(&lines2));
}

#[derive(Debug, Clone)]
struct Node {
    antinodes: Vec<char>,
}

impl Node {
    fn new(antinodes: Vec<char>) -> Self {
        Node {
            antinodes,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.antinodes.is_empty() {
            f.write_char(self.antinodes[0])
        } else {
            f.write_char('.')
        }
    }
}

fn part1(lines: &Vec<String>, ignore_distances: bool) -> usize {
    let mut freq_to_antennae: HashMap<char, Vec<Point>> = HashMap::new();
    let mut matrix: Vec<Vec<Node>> = lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    // Init antennae map
                    if c != '.' {
                        match freq_to_antennae.get_mut(&c) {
                            Some(v) => {
                                v.push(Point::new(x, y));
                            }
                            None => {
                                freq_to_antennae.insert(c, vec![Point::new(x, y)]);
                            }
                        }
                    }

                    // Init antinodes grid
                    Node::new(vec![])
                })
                .collect_vec()
        })
        .collect_vec();

    matrix.iter_mut().enumerate().for_each(|(y, line)| {
        line.iter_mut().enumerate().for_each(|(x, n)| {
            freq_to_antennae.iter().for_each(|(k, v)| {
                if *k != '.'
                    && v.len() > 1
                    && v.iter().combinations(2).any(|vec| {
                        let p = Point::new(x, y);
                        let (d1, d2) = (vec[0].distance(&p), vec[1].distance(&p));

                        let mut newvec = vec![&p];
                        newvec.extend(vec);

                        are_collinear(newvec.clone())
                            && (ignore_distances || (d1 == 2f64 * d2 || d2 == 2f64 * d1))
                    })
                {
                    n.antinodes.push(*k);
                }
            });
        });
    });

    matrix
        .iter()
        .map(|l| l.iter().filter(|n| !n.antinodes.is_empty()).count())
        .sum()
}

fn part2(lines: &Vec<String>) -> usize {
    part1(lines, true)
}
