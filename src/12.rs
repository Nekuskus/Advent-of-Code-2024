use debug_print::{debug_print as debug, debug_println as debugln};
use itertools::Itertools;
use setup_utils::*;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    path::Path,
};

// Symbols to replace: 12 1930 1206 1387004 SOLVE2

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/12-example.txt"));
        let result = crate::part1(&lines);
        if result == 1930 {
            Ok(())
        } else {
            Err(format!(
                "12: Bad result for Part 1 example, expected 1930 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/12-example.txt"));
        let result = crate::part2(&lines);
        if result == 1206 {
            Ok(())
        } else {
            Err(format!(
                "12: Bad result for Part 2 example, expected 1206 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/12-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);

        if result1 == 1387004 {
            Ok(())
        } else {
            Err(format!(
                "12: Bad result for Part 1, expected 1387004 got {}",
                result1
            ))
        }
        /*
        match (result1, result2) {
            (1387004, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!(
                "12: Bad result for Part 1, expected 1387004 got {}",
                result1
            )),
            (1387004, _) => Err(format!(
                "12: Bad result for Part 2, expected SOLVE2 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "12: Bad result for Part 1 & 2, expected (1387004, SOLVE2) got ({}, {})",
                result1, result2
            )),
        }*/
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/12-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/12-example.txt"));

    println!("12-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("12-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

#[derive(Debug, PartialEq)]
enum Align {
    Up,
    Down,
    Left,
    Right,
    Both,
}

fn count_sides(
    grid: &Vec<Vec<char>>,
    perimeter: &BTreeSet<PointI>,
    shape: &BTreeSet<PointI>,
) -> usize {
    let (xlen, ylen) = (grid[0].len() as isize, grid.len() as isize);
    let shape_p = shape.iter().next().unwrap();
    let _symbol = grid[shape_p.y as usize][shape_p.x as usize];

    let mut visited_horizontal = BTreeSet::new();
    let mut visited_vertical = BTreeSet::new();

    let mut sides = 0;

    for p in perimeter {
        let (mut len_horizontal, mut len_vertical) = (0, 0);
        let (mut horizontal_twice, mut vertical_twice) = (false, false);
        let mut queue = VecDeque::from(vec![*p]);

        if !visited_horizontal.contains(p)
                // bounds check left and right column
            && !(p.x == -1 || p.x == xlen)
        {
            debug!("h {_symbol} ");
            let mut align = Align::Both;

            while let Some(next) = queue.pop_front() {
                if !(next.y == -1 || next.y == 0)
                    && shape.contains(&PointI::new(next.x, next.y - 1))
                    && (align == Align::Up || align == Align::Both)
                {
                    // check up
                    align = Align::Up;

                    visited_horizontal.insert(next);
                    len_horizontal += 1;

                    let neighbors = [
                        PointI::new(next.x - 1, next.y),
                        PointI::new(next.x + 1, next.y),
                    ];

                    for n in neighbors {
                        if perimeter.contains(&n)
                            && !visited_horizontal.contains(&n)
                            && !queue.contains(&n)
                            && shape.contains(&PointI::new(n.x, n.y - 1))
                        {
                            queue.push_front(n);
                        }
                    }
                } else if !(next.y == ylen || next.y == ylen - 1)
                    && shape.contains(&PointI::new(next.x, next.y + 1))
                    && (align == Align::Down || align == Align::Both)
                {
                    // check down
                    align = Align::Down;

                    visited_horizontal.insert(next);
                    len_horizontal += 1;

                    let neighbors = [
                        PointI::new(next.x - 1, next.y),
                        PointI::new(next.x + 1, next.y),
                    ];

                    for n in neighbors {
                        if perimeter.contains(&n)
                            && !visited_horizontal.contains(&n)
                            && !queue.contains(&n)
                            && shape.contains(&PointI::new(n.x, n.y + 1))
                        {
                            queue.push_front(n);
                        }
                    }
                } else {
                    continue;
                }

                debug!("{next} ");
                if !(next.x == -1
                    || next.x == xlen
                    || next.y == -1
                    || next.y == 0
                    || next.y == ylen - 1
                    || next.y == ylen)
                    && shape.contains(&PointI::new(next.x, next.y - 1))
                    && shape.contains(&PointI::new(next.x, next.y + 1))
                {
                    // double side, count twice
                    horizontal_twice = true;
                }
            }
            if horizontal_twice {
                debug!("twice");
            }
            debugln!();
        }

        queue.clear();
        queue.push_front(*p);

        if !visited_vertical.contains(p)
                // bounds check top and bottom row
            && !(p.y ==  -1 || p.y == ylen)
        {
            debug!("v {_symbol} ");
            let mut align = Align::Both;

            while let Some(next) = queue.pop_front() {
                if !(next.x == -1 || next.x == 0)
                    && shape.contains(&PointI::new(next.x - 1, next.y))
                    && (align == Align::Left || align == Align::Both)
                {
                    // check left
                    align = Align::Left;

                    visited_vertical.insert(next);
                    len_vertical += 1;

                    let neighbors = [
                        PointI::new(next.x, next.y - 1),
                        PointI::new(next.x, next.y + 1),
                    ];

                    for n in neighbors {
                        if perimeter.contains(&n)
                            && !visited_vertical.contains(&n)
                            && !queue.contains(&n)
                            && shape.contains(&PointI::new(n.x - 1, n.y))
                        {
                            queue.push_front(n);
                        }
                    }
                } else if !(next.x == xlen || next.x == xlen - 1)
                    && shape.contains(&PointI::new(next.x + 1, next.y))
                    && (align == Align::Right || align == Align::Both)
                {
                    // check right
                    align = Align::Right;

                    visited_vertical.insert(next);
                    len_vertical += 1;

                    let neighbors = [
                        PointI::new(next.x, next.y - 1),
                        PointI::new(next.x, next.y + 1),
                    ];

                    for n in neighbors {
                        if perimeter.contains(&n)
                            && !visited_vertical.contains(&n)
                            && !queue.contains(&n)
                            && shape.contains(&PointI::new(n.x + 1, n.y))
                        {
                            queue.push_front(n);
                        }
                    }
                } else {
                    continue;
                }

                debug!("{next} ");
                if !(next.x == -1
                    || next.x == 0
                    || next.x == xlen - 1
                    || next.x == xlen
                    || next.y == -1
                    || next.y == ylen)
                    && shape.contains(&PointI::new(next.x - 1, next.y))
                    && shape.contains(&PointI::new(next.x + 1, next.y))
                {
                    // double side, count twice
                    vertical_twice = true;
                }
            }
            if vertical_twice {
                debug!("twice");
            }
            debugln!("");
        }

        if len_horizontal > 0 {
            if horizontal_twice {
                sides += 2
            } else {
                sides += 1
            }
        }

        if len_vertical > 0 {
            if vertical_twice {
                sides += 2
            } else {
                sides += 1
            }
        }
    }

    sides
}

fn process_region(
    grid: &Vec<Vec<char>>,
    visited: &mut BTreeSet<PointI>,
    p: &PointI,
    flatten_sides: bool,
) -> usize {
    let mut area = BTreeSet::new();
    let mut perimeter = BTreeMap::new();

    let (xlen, ylen) = (grid[0].len() as isize, grid.len() as isize);
    let symbol = grid[p.y as usize][p.x as usize];

    let mut queue = VecDeque::from(vec![PointI::new(p.x as isize, p.y as isize)]);

    while let Some(next) = queue.pop_front() {
        area.insert(next);

        let neighbors = [
            PointI::new(next.x - 1, next.y),
            PointI::new(next.x + 1, next.y),
            PointI::new(next.x, next.y - 1),
            PointI::new(next.x, next.y + 1),
        ];

        for n in neighbors {
            if n.x == -1
                || n.y == -1
                || n.x == xlen
                || n.y == ylen
                || grid[n.y as usize][n.x as usize] != symbol
            {
                // println!("{symbol} {next}");
                let conv = PointI::new(n.x as isize, n.y as isize);
                match perimeter.get_mut(&conv) {
                    Some(count) => *count += 1,
                    None => {
                        perimeter.insert(conv, 1);
                    }
                }
            } else if !queue.contains(&n) && !area.contains(&n) {
                queue.push_front(n);
            }
        }
    }

    let ret: usize;

    if flatten_sides {
        let perim = count_sides(&grid, &perimeter.keys().copied().collect(), &area);
        debugln!(
            "{symbol} {}\t: area {}, perimeter {} ",
            area.iter().next().unwrap(),
            area.len(),
            perim
        );
        ret = area.len() * perim;
    } else {
        ret = area.len() * perimeter.iter().map(|(_, v)| v).sum::<usize>();
    }

    visited.extend(area);

    ret
}

fn part1(lines: &Vec<String>) -> usize {
    let matrix = lines.iter().map(|s| s.chars().collect_vec()).collect_vec();
    let mut visited = BTreeSet::new();

    let mut sum = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let p = PointI::new(x as isize, y as isize);
            if !visited.contains(&p) {
                sum += process_region(&matrix, &mut visited, &p, false);
            }
        }
    }

    sum
}

fn part2(lines: &Vec<String>) -> usize {
    let matrix = lines.iter().map(|s| s.chars().collect_vec()).collect_vec();
    let mut visited = BTreeSet::new();

    let mut sum = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let p = PointI::new(x as isize, y as isize);
            if !visited.contains(&p) {
                sum += process_region(&matrix, &mut visited, &p, true);
            }
        }
    }

    sum
}
