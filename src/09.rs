use itertools::Itertools;
use setup_utils::*;
use std::{collections::VecDeque, path::Path};

// Symbols to replace: 09 1928 2858 6283404590840 6304576012713

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/09-example.txt"));
        let result = crate::part1(&lines);
        if result == 1928 {
            Ok(())
        } else {
            Err(format!(
                "09: Bad result for Part 1 example, expected 1928 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/09-example.txt"));
        let result = crate::part2(&lines);
        if result == 2858 {
            Ok(())
        } else {
            Err(format!(
                "09: Bad result for Part 2 example, expected 2858 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/09-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (6283404590840, 6304576012713) => Ok(()),
            (_, 6304576012713) => Err(format!(
                "09: Bad result for Part 1, expected 6283404590840 got {}",
                result1
            )),
            (6283404590840, _) => Err(format!(
                "09: Bad result for Part 2, expected 6304576012713 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "09: Bad result for Part 1 & 2, expected (6283404590840, 6304576012713) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/09-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/09-example.txt"));

    println!("09-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("09-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

#[derive(Debug)]
enum Index {
    File { id: u64, size: u64 },
    Empty { size: u64 },
}

fn part1(lines: &Vec<String>) -> u64 {
    // Output
    let mut line = lines[0]
        .chars()
        .enumerate()
        .map(|(idx, c)| match idx % 2 {
            0 => Index::File {
                id: (idx / 2) as u64,
                size: c.to_digit(10).unwrap() as u64,
            },
            1 => Index::Empty {
                size: c.to_digit(10).unwrap() as u64,
            },
            _ => unreachable!(),
        })
        .collect::<VecDeque<_>>();

    while let Some(empty_idx) = line
        .iter()
        .position(|entry| matches!(entry, Index::Empty { size: _ }))
    {
        // Avoiding multiple mutable borrows by using indices
        let back_idx = line.len() - 1;

        match line[back_idx] {
            Index::Empty { size: _ } => {
                line.pop_back();
            }
            Index::File {
                id: back_id,
                size: back_size,
            } => {
                match line[empty_idx] {
                    Index::Empty { size: empty_size } => {
                        if back_size > empty_size {
                            line[empty_idx] = Index::File {
                                id: back_id,
                                size: empty_size,
                            };
                            line[back_idx] = Index::File {
                                id: back_id,
                                size: back_size - empty_size,
                            }
                        } else if back_size == empty_size {
                            line[empty_idx] = Index::File {
                                id: back_id,
                                size: empty_size,
                            };
                            line.pop_back();
                        } else {
                            // back_size < empty_size
                            line[empty_idx] = Index::File {
                                id: back_id,
                                size: back_size,
                            };
                            line.insert(
                                empty_idx + 1,
                                Index::Empty {
                                    size: empty_size - back_size,
                                },
                            );
                            line.pop_back();
                        }
                    }
                    Index::File { id: _, size: _ } => unreachable!(),
                };
            }
        }
    }

    // line.iter().for_each(|x| match x {
    //     Index::Empty { size } => {
    //         print!("{}", ".".repeat(*size as usize));
    //     }
    //     Index::File { id, size } => {
    //         print!("{}", format!("{id}").repeat(*size as usize));
    //     }
    // });

    // println!();

    line.iter()
        .map(|item| match item {
            Index::Empty { size } => [&0].repeat(*size as usize),
            Index::File { id, size } => [id].repeat(*size as usize),
        })
        .flatten()
        .enumerate()
        .map(|(idx, size)| {
            // println!("{idx} * {size} == {}", idx as u32 * size);
            idx as u64 * size
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> u64 {
    // Output
    let mut line = lines[0]
        .chars()
        .enumerate()
        .map(|(idx, c)| match idx % 2 {
            0 => Index::File {
                id: (idx / 2) as u64,
                size: c.to_digit(10).unwrap() as u64,
            },
            1 => Index::Empty {
                size: c.to_digit(10).unwrap() as u64,
            },
            _ => unreachable!(),
        })
        .collect::<VecDeque<_>>();

    for source_idx in (0..line.len()).rev() {
        let empty = line
            .iter()
            .enumerate()
            .filter_map(|(idx, entry)| {
                (matches!(entry, Index::Empty { size: _ }) && idx < source_idx).then_some(idx)
            })
            .collect_vec();

        'swap: for dest_idx in empty {
            match line[source_idx] {
                Index::Empty { size: _ } => {}
                Index::File {
                    id,
                    size: dest_size,
                } => {
                    match line[dest_idx] {
                        Index::Empty { size: empty_size } => {
                            if dest_size == empty_size {
                                line[dest_idx] = Index::File {
                                    id,
                                    size: empty_size,
                                };
                                line[source_idx] = Index::Empty { size: dest_size };
                                break 'swap;
                            } else if dest_size < empty_size {
                                line[dest_idx] = Index::File {
                                    id,
                                    size: dest_size,
                                };
                                line[source_idx] = Index::Empty { size: dest_size };

                                line.insert(
                                    dest_idx + 1,
                                    Index::Empty {
                                        size: empty_size - dest_size,
                                    },
                                );
                                break 'swap;
                            }
                        }
                        Index::File { id: _, size: _ } => unreachable!(),
                    };
                }
            }
        }
    }

    line.iter()
        .map(|item| match item {
            Index::Empty { size } => [&0].repeat(*size as usize),
            Index::File { id, size } => [id].repeat(*size as usize),
        })
        .flatten()
        .enumerate()
        .map(|(idx, size)| {
            // println!("{idx} * {size} == {}", idx as u32 * size);
            idx as u64 * size
        })
        .sum()
}
