use setup_utils::*;
use std::path::Path;

// Symbols to replace: 10 8 TEST2 SOLVE1 SOLVE2


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/10-1-example.txt"));
        let result = crate::part1(&lines);
        if result == 8 {
            Ok(())
        } else {
            Err(format!("10: Bad result for Part 1 example, expected 8 got {}", result))
        }
    }
    /*
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/10-2-example.txt"));
        let result = crate::part2(&lines);
        if result == TEST2 {
            Ok(())
        } else {
            Err(format!("10: Bad result for Part 2 example, expected TEST2 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/10-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);
        
        if result1 == SOLVE1 {
            Ok(())
        } else {
            Err(format!("10: Bad result for Part 1, expected SOLVE1 got {}", result1))
        }
        /*
        match (result1, result2) {
            (SOLVE1, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("10: Bad result for Part 1, expected SOLVE1 got {}", result1)),
            (SOLVE1, _) => Err(format!("10: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("10: Bad result for Part 1 & 2, expected (SOLVE1, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
    */
}


#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
enum EntryDirection {
    north,
    south,
    west,
    east
}

struct Point {
    x: usize,
    y: usize
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

#[inline(always)]
fn get_next_point(x: usize, y: usize, symbol: char, dir: EntryDirection) -> (usize, usize, EntryDirection) {
    use EntryDirection::*;

    // Coordinate space: [y: 0..N][x: 0..N] from top-left
    match symbol {
        '|' => match dir {
            north => (x, y+1, north),
            south => (x, y-1, south),
            _ => panic!("Wrong entry dir for {symbol}: {dir:?}")
        }
        '-' => match dir {
            west => (x + 1, y, west),
            east => (x - 1, y, east),
            _ => panic!("Wrong entry dir for {symbol}: {dir:?}")
        }
        'L' => match dir {
            north => (x + 1, y, west),
            east => (x, y - 1, south),
            _ => panic!("Wrong entry dir for {symbol}: {dir:?}")
        }
        'J' => match dir {
            north => (x - 1, y, east),
            west => (x, y - 1, south),
            _ => panic!("Wrong entry dir for {symbol}: {dir:?}")
        }
        '7' => match dir {
            south => (x - 1, y, east),
            west => (x, y + 1, north),
            _ => panic!("Wrong entry dir for {symbol}: {dir:?}")
        }
        
        'F' => match dir {
            south => (x + 1, y, west),
            east => (x, y + 1, north),
            _ => panic!("Wrong entry dir for {symbol}: {dir:?}")
        }
        'S' => panic!("Input somehow looped back to start, which should not have happened: {symbol}"),
        '.' | _ => panic!("Went out of loop or unknown symbol: {symbol}")
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/10-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/10-1-example.txt"));
    //let lines2 = read_lines(Path::new("./inputs/10-2-example.txt"));

    println!("10-full.txt");
    println!("{}", part1(&linesfull));
    //println!("{}\n", part2(&linesfull));
    
    println!("10-1-example.txt");
    println!("{}", part1(&lines1));
    //println!("{}\n", part2(&lines1));
    
    
    //println!("10-2-example.txt");
    //println!("{}", part1(&lines2));
    //println!("{}", part2(&lines2));
    
}

fn part1(lines: &Vec<String>) -> i32 {
    use EntryDirection::*;

    let pipes: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let mut startpoint_temp: Option<Point> = None;
    for (y, line) in pipes.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'S' {
                startpoint_temp = Some(Point { x, y });
                break;
            }
        }
    }

    let startpoint = startpoint_temp.expect("Animal was not found in input!");

    // temp_y, temp_y; casting to i32 to prevent panic on underflow (value below 0)
    let tx = startpoint.x as i32;
    let ty = startpoint.y as i32;
    let indices = [                      (tx, ty-1, south),
                                                        (tx-1, ty, east),                      (tx+1, ty, west),
                                                                            (tx, ty+1, north),                      ];
    let mut possible_starts = indices.iter().filter_map(|(x, y, dir)|
    match x >= &0 && x < &(lines[0].len() as i32) &&
        y >= &0 && y < &(lines.len() as i32) {
        true => match pipes[*y as usize][*x as usize] {
            '.' => Some((&tx, &ty, north)), // this will be invalidated during pruning
            _ => Some((x, y, *dir)),
        },
        false => Some((&tx, &ty, north)) //this will be invalidated during pruning
    }).collect::<Vec<_>>();
    
    // Remove invalid connections from starting points
    let mut to_remove = vec![];

    //println!("{possible_starts:?}");

    if !['|', 'L', 'J'].contains(&pipes[*possible_starts[3].1 as usize][*possible_starts[3].0 as usize]) {
        to_remove.push(3_usize);
    }
    if !['-', 'J', '7'].contains(&pipes[*possible_starts[2].1 as usize][*possible_starts[2].0 as usize]) {
        to_remove.push(2_usize);
    }
    if !['-', 'L', 'F'].contains(&pipes[*possible_starts[1].1 as usize][*possible_starts[1].0 as usize]) {
        to_remove.push(1_usize);
    }
    if !['|', '7', 'F'].contains(&pipes[*possible_starts[0].1 as usize][*possible_starts[0].0 as usize]) {
        to_remove.push(0_usize);
    }
    for idx in to_remove {
        possible_starts.splice(idx..=idx, []);
    }

    assert_eq!(len!(possible_starts), 2);

    let mut p1 = Point {x: *possible_starts[0].0 as usize, y: *possible_starts[0].1 as usize};
    let mut p1_dir = possible_starts[0].2;
    let mut p2 = Point {x: *possible_starts[1].0 as usize, y: *possible_starts[1].1 as usize};
    let mut p2_dir = possible_starts[1].2;

    let mut cur_distance = 1;
    let mut _prev_s1: char = 'S';
    let mut _prev_s2: char = 'S';

    while p1 != p2 {
        let cur_s1 = pipes[p1.y][p1.x];
        let cur_s2 = pipes[p2.y][p2.x];
        //println!("{cur_s1}");
        (p1.x, p1.y, p1_dir) = get_next_point(p1.x, p1.y, pipes[p1.y][p1.x], p1_dir);
        (p2.x, p2.y, p2_dir) = get_next_point(p2.x, p2.y, pipes[p2.y][p2.x], p2_dir);
        cur_distance += 1;
        _prev_s1 = cur_s1;
        _prev_s2 = cur_s2;
    }

    return cur_distance;
}
/*
fn part2(lines: &Vec<String>) -> i32 {

}
*/