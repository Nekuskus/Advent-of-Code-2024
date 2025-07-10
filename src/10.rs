use debug_print::{debug_print as debug, debug_println as debugln};
use setup_utils::*;
use std::path::Path;

// Symbols to replace: 10 8 10 6768 SOLVE2


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
    
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/10-2-example.txt"));
        let result = crate::part2(&lines);
        if result == 4 {
            Ok(())
        } else {
            Err(format!("10: Bad result for Part 2 example, expected 4 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/10-full.txt"));
        let result1 = crate::part1(&lines);
        //let result2 = crate::part2(&lines);
        
        if result1 == 6768 {
            Ok(())
        } else {
            Err(format!("10: Bad result for Part 1, expected 6768 got {}", result1))
        }
        /*
        match (result1, result2) {
            (6768, SOLVE2) => Ok(()),
            (_, SOLVE2) => Err(format!("10: Bad result for Part 1, expected 6768 got {}", result1)),
            (6768, _) => Err(format!("10: Bad result for Part 2, expected SOLVE2 got {}", result2)),
            (_, _) => Err(format!("10: Bad result for Part 1 & 2, expected (6768, SOLVE2) got ({}, {})", result1, result2))
        }*/
    }
    
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IsChecked {
    Inside,
    Outside,
    Pending,
    Unset
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
enum EntryDirection {
    north,
    south,
    west,
    east
}

#[derive(Debug)]
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
    let lines2 = read_lines(Path::new("./inputs/10-2-example.txt"));

    println!("10-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));
    
    println!("10-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
    
    
    println!("10-2-example.txt");
    println!("{}", part1(&lines2));
    println!("{}", part2(&lines2));
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

    //debugln!("{possible_starts:?}");

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
        //debugln!("{cur_s1}");
        (p1.x, p1.y, p1_dir) = get_next_point(p1.x, p1.y, pipes[p1.y][p1.x], p1_dir);
        (p2.x, p2.y, p2_dir) = get_next_point(p2.x, p2.y, pipes[p2.y][p2.x], p2_dir);
        cur_distance += 1;
        _prev_s1 = cur_s1;
        _prev_s2 = cur_s2;
    }

    return cur_distance;
}

fn part2(lines: &Vec<String>) -> i32 {
    use EntryDirection::*;
    use IsChecked::*;

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
    let mut valids = vec![];

    //debugln!("{possible_starts:?}");

    if !['|', 'L', 'J'].contains(&pipes[*possible_starts[3].1 as usize][*possible_starts[3].0 as usize]) {
        to_remove.push(3_usize);
    } else {
        valids.push(3_usize);
    }
    if !['-', 'J', '7'].contains(&pipes[*possible_starts[2].1 as usize][*possible_starts[2].0 as usize]) {
        to_remove.push(2_usize);
    } else {
        valids.push(2_usize);
    }
    if !['-', 'L', 'F'].contains(&pipes[*possible_starts[1].1 as usize][*possible_starts[1].0 as usize]) {
        to_remove.push(1_usize);
    } else {
        valids.push(1_usize);
    }
    if !['|', '7', 'F'].contains(&pipes[*possible_starts[0].1 as usize][*possible_starts[0].0 as usize]) {
        to_remove.push(0_usize);
    } else {
        valids.push(0_usize);
    }
    for idx in to_remove {
        possible_starts.splice(idx..=idx, []);
    }

    assert_eq!(len!(possible_starts), 2);
    assert_eq!(len!(valids), 2);

    let mut p1 = Point {x: *possible_starts[0].0 as usize, y: *possible_starts[0].1 as usize};
    let mut p1_dir = possible_starts[0].2;
    let mut p2 = Point {x: *possible_starts[1].0 as usize, y: *possible_starts[1].1 as usize};
    let mut p2_dir = possible_starts[1].2;

    
    //  PART 2 PROPER STARTS HERE
    let mut pipe_traced = vec![vec!['.'; len!(pipes[0])]; len!(pipes)];
    let mut checked = vec![vec![Unset; len!(pipes[0])]; len!(pipes)];

    /*
        south: 3
        east:  2
        west:  1
        north: 0
    */
    pipe_traced[startpoint.y][startpoint.x] = match valids[..] {
        [3, 0] => '|',
        [2, 1] => '-',
        [2, 0] => 'L',
        [1, 0] => 'J',
        [3, 1] => '7',
        [3, 2] => 'F',
        [] | [..] => panic!("S does not match any known pattern! {valids:?}")
    };

    while p1 != p2 {
        pipe_traced[p1.y][p1.x] = pipes[p1.y][p1.x];
        pipe_traced[p2.y][p2.x] = pipes[p2.y][p2.x];
        (p1.x, p1.y, p1_dir) = get_next_point(p1.x, p1.y, pipes[p1.y][p1.x], p1_dir);
        (p2.x, p2.y, p2_dir) = get_next_point(p2.x, p2.y, pipes[p2.y][p2.x], p2_dir);
    }

    pipe_traced[p1.y][p1.x] = pipes[p1.y][p1.x]; // Same point for p1 and p2

    // Find and map wormholes: mapping points between (x1, y1) <-> (x2, y2) through squeeze tunnels
    let mut wormhole_starts: Vec<(i32, i32)> = vec![];
    let mut wormhole_ends: Vec<(i32, i32)> = vec![];

    for (y, line) in pipe_traced.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'.' && !wormhole_ends.contains(&(x as i32, y as i32)) {
                // Check top row
                if y > 0 {
                    if x > 0 {
                        let s: String = [pipe_traced[y-1][x-1], pipe_traced[y-1][x]].iter().collect();
                        if s == "|L" {
                            let res = check_wormhole(x as i32, y as i32 - 1, s.chars().collect(), &pipe_traced, north, &wormhole_ends);
                            debugln!("Obtained res: {res:?}");
                            if res.is_some() {
                                let coords = res.unwrap();
                                wormhole_starts.push(coords.0);
                                wormhole_ends.push(coords.1);
                            }
                        }
                    }
                    if x + 1 < line.len() {
                        let s: String = [pipe_traced[y-1][x], pipe_traced[y-1][x+1]].iter().collect();
                        if ["JL", "J|", "||"].contains(&&s[..]) {
                            let res = check_wormhole(x as i32, y as i32 - 1, s.chars().collect(), &pipe_traced, north, &wormhole_ends);
                            debugln!("Obtained res: {res:?}");
                            if res.is_some() {
                                let coords = res.unwrap();
                                wormhole_starts.push(coords.0);
                                wormhole_ends.push(coords.1);
                            }
                        }
                    }
                }

                // Check bottom row
                if y + 1 < pipe_traced.len() {
                    if x > 0 {
                        let s: String = [pipe_traced[y+1][x-1], pipe_traced[y+1][x]].iter().collect();
                        if s == "|F" {
                            let res = check_wormhole(x as i32, y as i32 + 1, s.chars().collect(), &pipe_traced, south, &wormhole_ends);
                            debugln!("Obtained res: {res:?}");
                            if res.is_some() {
                                let coords = res.unwrap();
                                wormhole_starts.push(coords.0);
                                wormhole_ends.push(coords.1);
                            }
                        }
                    }
                    if x + 1 < line.len() {
                        let s: String = [pipe_traced[y+1][x], pipe_traced[y+1][x+1]].iter().collect();
                        if ["7F", "7|", "||"].contains(&&s[..]) {
                            let res = check_wormhole(x as i32, y as i32 + 1, s.chars().collect(), &pipe_traced, south, &wormhole_ends);
                            debugln!("Obtained res: {res:?}");
                            if res.is_some() {
                                let coords = res.unwrap();
                                wormhole_starts.push(coords.0);
                                wormhole_ends.push(coords.1);
                            }
                        }
                    }
                }

                // Check left row
                if x > 0 {
                    if y > 0 {
                        let s: String = [pipe_traced[y-1][x-1], pipe_traced[y][x-1]].iter().collect();
                        if s == "-7" {
                            let res = check_wormhole(x as i32 - 1, y as i32, s.chars().collect(), &pipe_traced, west, &wormhole_ends);
                            debugln!("Obtained res: {res:?}");
                            if res.is_some() {
                                let coords = res.unwrap();
                                wormhole_starts.push(coords.0);
                                wormhole_ends.push(coords.1);
                            }
                        }
                    }
                    if y + 1 < pipe_traced.len() {
                        let s: String = [pipe_traced[y][x-1], pipe_traced[y+1][x-1]].iter().collect();
                        if ["J7", "--", "J-"].contains(&&s[..]) {
                            let res = check_wormhole(x as i32 - 1, y as i32, s.chars().collect(), &pipe_traced, west, &wormhole_ends);
                            debugln!("Obtained res: {res:?}");
                            if res.is_some() {
                                let coords = res.unwrap();
                                wormhole_starts.push(coords.0);
                                wormhole_ends.push(coords.1);
                            }
                        }
                    }
                }


                // Check right row
                if x + 1 < line.len() {
                    if y > 0 {
                        let s: String = [pipe_traced[y-1][x+1], pipe_traced[y][x+1]].iter().collect();
                        if s == "-F" {
                            let res = check_wormhole(x as i32 + 1, y as i32, s.chars().collect(), &pipe_traced, east, &wormhole_ends);
                            debugln!("Obtained res: {res:?}");
                            if res.is_some() {
                                let coords = res.unwrap();
                                wormhole_starts.push(coords.0);
                                wormhole_ends.push(coords.1);
                            }
                        }
                    }
                    if y + 1 < pipe_traced.len() {
                        let s: String = [pipe_traced[y][x+1], pipe_traced[y+1][x+1]].iter().collect();
                        if ["LF", "--", "L-"].contains(&&s[..]) {
                            let res = check_wormhole(x as i32 + 1, y as i32, s.chars().collect(), &pipe_traced, east, &wormhole_ends);
                            debugln!("Obtained res: {res:?}");
                            if res.is_some() {
                                let coords = res.unwrap();
                                wormhole_starts.push(coords.0);
                                wormhole_ends.push(coords.1);
                            }
                        }
                    }
                }
            }
        }
    }

    for line in &pipe_traced {
        println!("{}", line.iter().collect::<String>());
    }

    // debugln!("------------------------------------------");
    for (y, line) in pipe_traced.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c != &'*' {
                start_checking(x as i32, y as i32, &mut checked, &pipe_traced, &wormhole_starts, &wormhole_ends);
                //debugln!("0123456789");
                for (y,line) in checked.iter().enumerate() {
                    for (x, val) in line.iter().enumerate() {
                        // debug!("{}", match val {
                        //     Inside => 'I',
                        //     Outside => 'O',
                        //     Unset => pipe_traced[y][x],
                        //     Pending => panic!("Point ({x}, {y}) did not get evaluated before returning from outer checker!")
                        // })
                    }
                    // debugln!();
                }
                // debugln!("------------------------------------------");
            }
        }
    
    }
    
    for (y,line) in checked.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            print!("{}", match val {
                Inside => 'I',
                Outside => 'O',
                Unset => match pipe_traced[y][x] {
                    '|' => '│',
                    '-' => '─',
                    'L' => '└',
                    'J' => '┘',
                    '7' => '┐',
                    'F' => '┌',
                    _ => pipe_traced[y][x]
                },
                Pending => panic!("Point ({x}, {y}) did not get evaluated before returning from outer checker!")
            });
        }
        println!()
    }

    let mut inside_counter = 0;

    for line in checked {
        for val in line {
            if val == Inside {
                inside_counter += 1;
            }
        }
    }
    
    debugln!("wormhole starts: {wormhole_starts:?}");
    debugln!("wormhole ends:   {wormhole_ends:?}");

    return inside_counter;
}

fn check_wormhole(startx: i32, starty: i32, mut entrance: Vec<char>,  pipes: &Vec<Vec<char>>, mut dir: EntryDirection, wormhole_ends: &Vec<(i32, i32)>) -> Option<((i32, i32), (i32, i32))> {
    if wormhole_ends.contains(&(startx, starty)) {
        return None;
    }

    use EntryDirection::*;
    // About arguments x and y, and entrance
    // When a wormhole starts with:
    // (upwards, north)
    // JL, J|, || always call this function with the LEFT character horizontally TODO ADD: JF
    // |L         always call this function with the RIGHT character horizontally
    // (downwards, south)
    // 7F, 7|, || always call this function with the LEFT character horizontally TODO ADD: JF
    // |F         always call this function with the RIGHT character horizontally
    // (leftward, west) entrance order: top to bottom
    // J  -   J         always call with the TOP character vertically
    // 7  -   - 
    // -                always call with the BOTTOM character vertically
    // 7
    // (rightward, east) entrance order: top to bottom
    // L  -   L         always call with the TOP character vertically  TODO ADD: J
    // F  -   -                                                                  F
    // -                always call with the BOTTOM character vertically
    // F
    // Will not go out of bounds for given input file
    let mut p1 = Point {x: startx as usize, y: starty as usize};
    let mut p2 = match entrance.as_slice() {
        ['J', 'L'] | ['J', '|'] | ['|', '|'] | ['7', 'F'] | ['7', '|'] => Point { x: p1.x + 1, y: p1.y },
        ['|', 'L'] | ['|', 'F']  => Point { x: p1.x - 1, y: p1.y },
        ['J', '7'] | ['-', '-'] | ['J', '-'] | ['L', 'F'] | ['L', '-'] => Point { x: p1.x, y: p1.y + 1 },
        ['-', '7'] | ['-', 'F'] => Point { x: p1.x, y: p1.y - 1 },
        _ => panic!("Bad match in wormhole checker: {entrance:?}")
    };

    let northconnects = ['|', 'L', 'J'];
    let southconnects = ['|', '7', 'F'];
    let westconnects = ['-', 'J', '7'];
    let eastconnects = ['-', 'L', 'F'];

    debugln!();
    debugln!("--------------------------");
    debugln!("checking ({startx}, {starty})");
    debugln!("{entrance:?} dir={dir:?}");
    debugln!("[{}, {}]", pipes[p1.y][p1.x], pipes[p2.y][p2.x]);

    loop { // return with end coords when wormhole ends, return None if wormhole has no proper exits (no dots to iterate through)
        // Swap so that handling multiple orientations is not necessary, this tree is already as long as is
        if p2.x < p1.x {
            (p1.x, p2.x) = (p2.x, p1.x);
        }
        if p2.y < p1.y {
            (p1.y, p2.y) = (p2.y, p1.y);
        }
        debugln!("checking p1={p1:?} p2={p2:?} dir={dir:?}");
        debugln!("\t\t[{}, {}]", pipes[p1.y][p1.x], pipes[p2.y][p2.x]);
        match [pipes[p1.y][p1.x], pipes[p2.y][p2.x]] {
            ['J', 'L'] => {
                if dir == north { // 1 upwards
                    p1.y -= 1;
                    p2.y -= 1;
                } else if dir == south { // 1 downwards, possible ending
                    p1.y += 1;
                    p2.y += 1;
                    if pipes[p1.y][p1.x] == '.' {
                        return Some(((startx, starty), (p1.x as i32, p1.y as i32)));
                    } else if pipes[p2.y][p2.x] == '.' {
                        return Some(((startx, starty), (p2.x as i32, p2.y as i32)));
                    }
                } else {
                    return None
                }
            },
            ['J', '|'] => { 
                if dir == north { // 1 upwards
                    p1.y -= 1;
                    p2.y -= 1;
                } else if dir == south { // 1 downwards, possible ending, possible corner
                    p1.y += 1;
                    p2.y += 1;
                    if pipes[p1.y][p1.x] == '.' {
                        return Some(((startx, starty), (p1.x as i32, p1.y as i32)));
                    }
                    if pipes[p2.y][p2.x] == 'J' && westconnects.contains(&pipes[p2.y][p2.x - 1]) {
                        p1.y -= 1;
                        p2.x -= 1;
                        dir = west;
                    }
                } else {
                    return None
                }
            },
            ['|', '|'] => {
                if dir == north { // 1 upwards
                    p1.y -= 1;
                    p2.y -= 1;
                } else if dir == south { // 1 downwards
                    p1.y += 1;
                    p2.y += 1;
                } else {
                    return None
                }
            },
            ['7', 'F'] => {
                if dir == north { // 1 upwards, possible ending
                    p1.y -= 1;
                    p2.y -= 1;
                    if pipes[p1.y][p1.x] == '.' {
                        return Some(((startx, starty), (p1.x as i32, p1.y as i32)));
                    } else if pipes[p2.y][p2.x] == '.' {
                        return Some(((startx, starty), (p2.x as i32, p2.y as i32)));
                    }
                } else if dir == south { // 1 downwards
                    p1.y += 1;
                    p2.y += 1;
                } else {
                    return None
                }
            },
            ['7', '|'] => {
                if dir == north { // 1 upwards, possible ending, possible corner
                    p1.y -= 1;
                    p2.y -= 1;
                    if pipes[p1.y][p1.x] == '.' {
                        return Some(((startx, starty), (p1.x as i32, p1.y as i32)));
                    }
                    if pipes[p2.y][p2.x] == '7' && westconnects.contains(&pipes[p2.y][p2.x - 1]) {
                        p1.y += 1;
                        p2.x -= 1;
                        dir = west;
                    }
                } else if dir == south { // 1 downwards
                    p1.y += 1;
                    p2.y += 1;
                } else {
                    return None
                }
            },
            ['|', 'L'] => {
                if dir == north { // 1 upwards
                    p1.y -= 1;
                    p2.y -= 1;
                } else if dir == south { // 1 downwards, possible ending, possible corner
                    p1.y += 1;
                    p2.y += 1;
                    if pipes[p2.y][p2.x] == '.' {
                        return Some(((startx, starty), (p2.x as i32, p2.y as i32)));
                    }
                    if pipes[p1.y][p1.x] == 'L' && eastconnects.contains(&pipes[p1.y][p1.x + 1]) {
                        p2.y -= 1;
                        p1.x += 1;
                        dir = east;
                    }
                } else {
                    return None
                }
            },
            ['|', 'F'] => {
                if dir == north { // 1 upwards, possible ending, possible corner
                    p1.y -= 1;
                    p2.y -= 1;
                    if pipes[p2.y][p2.x] == '.' {
                        return Some(((startx, starty), (p2.x as i32, p2.y as i32)));
                    }
                    if pipes[p1.y][p1.x] == 'F' && eastconnects.contains(&pipes[p1.y][p1.x + 1]) {
                        p2.y += 1;
                        p1.x += 1;
                        dir = east;
                    }
                } else if dir == south { // 1 downwards
                    p1.y += 1;
                    p2.y += 1;
                } else {
                    return None
                }
            }
            ['J', '7'] => {
                if dir == east { // 1 rightward, possible ending
                    p1.x += 1;
                    p2.x += 1;
                    if pipes[p1.y][p1.x] == '.' {
                        return Some(((startx, starty), (p1.x as i32, p1.y as i32)));
                    } else if pipes[p2.y][p2.x] == '.' {
                        return Some(((startx, starty), (p2.x as i32, p2.y as i32)));
                    }
                } else if dir == west { // 1 leftward
                    p1.x -= 1;
                    p2.x -= 1;
                } else {
                    return None;
                }
            },
            ['-', '-'] => {
                if dir == east { // 1 rightward
                    p1.x += 1;
                    p2.x += 1;
                } else if dir == west { // 1 leftward
                    p1.x -= 1;
                    p2.x -= 1;
                } else {
                    return None;
                }
            },
            ['J', '-'] => {
                if dir == west { // 1 leftward
                    p1.x -= 1;
                    p2.x -= 1;
                } else if dir == east { // 1 rightward, possible ending, possible corner
                    p1.x += 1;
                    p2.x += 1;
                    if pipes[p1.y][p1.x] == '.' {
                        return Some(((startx, starty), (p1.x as i32, p1.y as i32)));
                    }
                    if pipes[p2.y][p2.x] == 'J' && northconnects.contains(&pipes[p2.y - 1][p2.x]) {
                        p2.y -= 1;
                        p1.x -= 1;
                        dir = north;
                    }
                } else {
                    return None
                }
            },
            ['-', '7'] => {
                if dir == west { // 1 leftward
                    p1.x -= 1;
                    p2.x -= 1;
                } else if dir == east { // 1 rightward, possible ending, possible corner
                    p1.x += 1;
                    p2.x += 1;
                    if pipes[p2.y][p2.x] == '.' {
                        return Some(((startx, starty), (p2.x as i32, p2.y as i32)));
                    }
                    if pipes[p1.y][p1.x] == '7' && southconnects.contains(&pipes[p1.y + 1][p1.x]) {
                        p1.y += 1;
                        p2.x -= 1;
                        dir = south;
                    }
                } else {
                    return None
                }
            },
            ['L', 'F'] => {
                if dir == east { // 1 leftward
                    p1.x += 1;
                    p2.x += 1;
                }
                else if dir == west { // 1 rightward, possible ending
                    p1.x -= 1;
                    p2.x -= 1;
                    if pipes[p1.y][p1.x] == '.' {
                        return Some(((startx, starty), (p1.x as i32, p1.y as i32)));
                    } else if pipes[p2.y][p2.x] == '.' {
                        return Some(((startx, starty), (p2.x as i32, p2.y as i32)));
                    }
                } else {
                    return None
                }
            },
            ['L', '-'] => {
                if dir == east { // 1 rightward
                    p1.x += 1;
                    p2.x += 1;
                } else if dir == west { // 1 leftward, possible ending, possible corner
                    p1.x -= 1;
                    p2.x -= 1;
                    if pipes[p1.y][p1.x] == '.' {
                        return Some(((startx, starty), (p1.x as i32, p1.y as i32)));
                    }
                    if pipes[p2.y][p2.x] == 'L' && northconnects.contains(&pipes[p2.y - 1][p2.x]) {
                        p2.y -= 1;
                        p1.x += 1;
                        dir = north;
                    }
                } else {
                    return None
                }
            }
            ['-', 'F'] => {
                if dir == east { // 1 rightward
                    p1.x += 1;
                    p2.x += 1;
                } else if dir == west { // 1 leftward, possible ending, possible corner
                    p1.x -= 1;
                    p2.x -= 1;
                    if pipes[p2.y][p2.x] == '.' {
                        return Some(((startx, starty), (p2.x as i32, p2.y as i32)));
                    }
                    if pipes[p1.y][p1.x] == 'F' && southconnects.contains(&pipes[p1.y + 1][p1.x]) {
                        p1.y += 1;
                        p2.x += 1;
                        dir = south;
                    }
                } else {
                    return None
                }
            }
            _ => return None // Wormhole ended nonsensically without reaching dots, wormhole invalid
        }
    }

    //return ((startx, starty), (endx, endy))
    // (endx, endy)
    //return Some(((0,0),(0,0)));
    // None if tunnel goes nowhere.
}

fn start_checking(x: i32, y: i32, checked: &mut Vec<Vec<IsChecked>>, pipes: &Vec<Vec<char>>, wormhole_starts: &Vec<(i32, i32)>, wormhole_ends: &Vec<(i32, i32)>) -> IsChecked {
    use IsChecked::*;
    let status = recursive_checker(x, y, checked, pipes, wormhole_starts, wormhole_ends);

    if [Inside, Outside, Unset].contains(&status) {
        return status;
    }

    // status == Pending

    for line in checked.iter_mut() {
        for iter_status in line.iter_mut() {
            if *iter_status == Pending {
                *iter_status = Inside;
            }
        }
    }
    return Inside;
}

// false if point not inside, true otherwise
// note to self: use vec.get() instead of [] to avoid panic on out of bounds access
#[inline(always)]
fn recursive_checker(x: i32, y: i32, checked: &mut Vec<Vec<IsChecked>>, pipes: &Vec<Vec<char>>, wormhole_starts: &Vec<(i32, i32)>, wormhole_ends: &Vec<(i32, i32)>) -> IsChecked {
    //debugln!("iterating ({x}, {y})");
    use IsChecked::*;
    let pipekinds = ['|', '-', 'F', 'J', '7', 'L'];
    
    if x < 0 || x >= pipes[0].len() as i32 || y < 0 || y >= pipes.len() as i32 {
        return Outside;
    }

    if pipekinds.contains(&pipes.get(y as usize).unwrap().get(x as usize).unwrap()) {
        return Unset;
    }

    /*if pipes.get(y as usize).unwrap_or(&vec![]).get(x as usize).is_none() {
        return False;
    }*/

    if checked[y as usize][x as usize] == Pending {
        return Pending;
    }


    if checked[y as usize][x as usize] == Outside {
        return Outside;
    }

    if checked[y as usize][x as usize] == Inside {
        return Inside;
    }

    let basic_neighbors: [(i32, i32); 4] = [
        (x,     y - 1),
        (x - 1, y),
        (x + 1, y),
        (x,     y + 1)
    ];

    let neighbors = basic_neighbors.map(|tup| {
        let position = wormhole_starts.iter().position(|x| x == &tup);
        match position {
            Some(idx) => wormhole_ends[idx],
            None => tup
        }
    });

    if neighbors.iter().any(|(_x, _y)| (_x < &0 || _x >= &(pipes[0].len() as i32)|| _y < &0 || _y >= &(pipes.len() as i32)) || checked[*_y as usize][*_x as usize] == Outside) {
        checked[y as usize][x as usize] = Outside;
        return Outside;
    }

    if neighbors.iter().any(|(_x, _y)| checked[*_y as usize][*_x as usize] == Inside) {
        checked[y as usize][x as usize] = Inside;
        return Inside;
    }

    checked[y as usize][x as usize] = Pending;
    for (_x, _y) in neighbors {
        if _x < 0 || _x >= pipes[0].len() as i32 || _y < 0 || _y >= pipes.len() as i32 {
            checked[y as usize][x as usize] = Outside;
            return Outside;
        }
        if checked[_y as usize][_x as usize] != Pending {
            let res = recursive_checker(_x, _y, checked, pipes, wormhole_starts, wormhole_ends);
            if res == Outside {
                neighbors.iter().for_each(|(__x, __y)| {
                    if !pipekinds.contains(pipes.get(*__y as usize).unwrap_or(&vec![]).get(*__x as usize).unwrap()) {
                        checked[*__y as usize][*__x as usize] = Outside;
                    }
                });

                checked[y as usize][x as usize] = Outside;
                return Outside;
            } else if res == Inside {
                neighbors.iter().for_each(|(__x, __y)| {
                    if !pipekinds.contains(&pipes.get(*__y as usize).unwrap_or(&vec![]).get(*__x as usize).unwrap()) {
                        checked[*__y as usize][*__x as usize] = Inside;
                    }
                });
                
                checked[y as usize][x as usize] = Inside;
                return Inside;
            }
        }
    }
    
    
    checked[y as usize][x as usize] = Pending;
    return Pending;
    
    
    /*
    if neighbors.iter().all(|(_x, _y)| {
        pipekinds.contains(&pipes.get(*_y as usize).unwrap_or(&vec![]).get(*_x as usize).unwrap()) || checked[*_y as usize][*_x as usize] == Pending
    }) {

    } else {
        checked[y as usize][x as usize] = True;
        return True;
    }
    */

}