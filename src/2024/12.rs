use itertools::Itertools;
use utils::*;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    path::Path,
};

// Symbols to replace: 12 1930 1206 1387004 844198

#[cfg(test)]
mod tests {
    use utils::get_input;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = get_input!("12-example.txt");
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
        let lines = get_input!("12-example.txt");
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
        let lines = get_input!("12-full.txt");
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (1387004, 844198) => Ok(()),
            (_, 844198) => Err(format!(
                "12: Bad result for Part 1, expected 1387004 got {}",
                result1
            )),
            (1387004, _) => Err(format!(
                "12: Bad result for Part 2, expected 844198 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "12: Bad result for Part 1 & 2, expected (1387004, 844198) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = get_input!("12-full.txt");
    let lines1 = get_input!("12-example.txt");

    println!("12-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("12-example.txt");
    println!("{}", part1(&lines1));
    println!("{}", part2(&lines1));
}

fn process_region(
    grid: &Vec<Vec<char>>,
    visited: &mut BTreeSet<PointI>,
    p: &PointI,
) -> (BTreeSet<PointI>, BTreeMap<PointI, usize>) {
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

    visited.extend(area.clone());

    (area, perimeter)
}

fn part1(lines: &Vec<String>) -> usize {
    let matrix = lines.iter().map(|s| s.chars().collect_vec()).collect_vec();
    let mut visited = BTreeSet::new();

    let mut sum = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let p = PointI::new(x as isize, y as isize);
            if !visited.contains(&p) {
                let (area, perimeter) = process_region(&matrix, &mut visited, &p);
                sum += area.len() * perimeter.iter().map(|(_, v)| v).sum::<usize>();
            }
        }
    }

    sum
}

fn scan_for_sides(
    grid: &Vec<Vec<char>>,
    area: &BTreeSet<PointI>,
    perimeter: &BTreeMap<PointI, usize>,
) -> usize {
    let (ylen, xlen) = (grid.len(), grid[0].len());

    // Index mapping:
    // 0                -> -1 [outer]
    // 1                -> 0 [first accessible]
    // ..
    // vec.len()        -> vec.len() - 1 [last accessible]
    // vec.len() + 1    -> vec.len() [outer]
    // Direction describes direction from perimeter to the area point
    // Technically one edge value in each vec will always be empty, but I prefer consistent indexing over that.
    let (mut rows_up, mut rows_down, mut columns_left, mut columns_right) = (
        vec![vec![false; xlen + 2]; ylen + 2],
        vec![vec![false; xlen + 2]; ylen + 2],
        vec![vec![false; xlen + 2]; ylen + 2],
        vec![vec![false; xlen + 2]; ylen + 2],
    );

    // Outer passes
    // for x in 0..xlen {
    //     if perimeter.contains_key(&PointI::new(x as isize, -1)) {
    //         rows_down[0][x] = true;
    //     }

    //     if perimeter.contains_key(&PointI::new(x as isize, ylen as isize)) {
    //         rows_up[ylen + 1][x] = true;
    //     }
    // }
    // for y in 0..ylen {
    //     if perimeter.contains_key(&PointI::new(-1, y as isize)) {
    //         columns_right[y][0] = true;
    //     }
    //     if perimeter.contains_key(&PointI::new(xlen as isize, y as isize)) {
    //         columns_left[y][xlen + 1] = true;
    //     }
    // }

    // // Inner pass
    // for (y, line) in grid.iter().enumerate() {
    //     for (x, c) in line.iter().enumerate() {}
    // }

    for (&p, count) in perimeter {
        let mut mut_count = *count;
        if area.contains(&PointI::new(p.x - 1, p.y)) && mut_count > 0 {
            columns_left[(p.x + 1) as usize][(p.y + 1) as usize] = true;
            mut_count -= 1;
        }
        if area.contains(&PointI::new(p.x + 1, p.y)) && mut_count > 0 {
            columns_right[(p.x + 1) as usize][(p.y + 1) as usize] = true;
            mut_count -= 1;
        }
        if area.contains(&PointI::new(p.x, p.y - 1)) && mut_count > 0 {
            rows_up[(p.y + 1) as usize][(p.x + 1) as usize] = true;
            mut_count -= 1;
        }
        if area.contains(&PointI::new(p.x, p.y + 1)) && mut_count > 0 {
            rows_down[(p.y + 1) as usize][(p.x + 1) as usize] = true;
            mut_count -= 1;
        }
        assert_eq!(mut_count, 0);
    }

    [rows_up, rows_down, columns_left, columns_right]
        .iter()
        .map(|vs| {
            vs.iter()
                .map(|v| v.split(|b| !b).filter(|v| !v.is_empty()).count())
                .sum::<usize>()
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> usize {
    let matrix = lines.iter().map(|s| s.chars().collect_vec()).collect_vec();
    let mut visited = BTreeSet::new();

    let mut sum = 0;

    for (y, line) in matrix.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let p = PointI::new(x as isize, y as isize);
            if !visited.contains(&p) {
                let (area, perimeter) = process_region(&matrix, &mut visited, &p);
                let sides = scan_for_sides(&matrix, &area, &perimeter);
                // let symbol =
                //     matrix[area.first().unwrap().y as usize][area.first().unwrap().x as usize];
                // println!("{symbol}\t: area {}, sides {}", area.len(), sides);
                sum += area.len() * sides;
            }
        }
    }

    sum
}
