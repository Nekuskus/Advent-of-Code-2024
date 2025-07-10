use setup_utils::read_lines;
use std::path::Path;
use std::collections::HashMap;


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/02-example.txt"));
        let result = crate::part1(&lines);
        if result == 8 {
            Ok(())
        } else {
            Err(format!("02: Bad result for Part 1 example, expected 8 got {}", result))
        }
    }
    
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/02-example.txt"));
        let result = crate::part2(&lines);
        if result == 2286 {
            Ok(())
        } else {
            Err(format!("02: Bad result for Part 2 example, expected 2286 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/02-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);
        match (result1, result2) {
            (2085, 79315) => Ok(()),
            (_, 79315) => Err(format!("02: Bad result for Part 1, expected 2085 got {}", result1)),
            (2085, _) => Err(format!("02: Bad result for Part 2, expected 79315 got {}", result2)),
            (_, _) => Err(format!("02: Bad result for Part 1 & 2, expected (2085, 79315) got ({}, {})", result1, result2))
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/02-full.txt"));
    let lines1and2 = read_lines(Path::new("./inputs/02-example.txt"));
    
    println!("02-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));
    
    println!("02-1-example.txt");
    println!("{}", part1(&lines1and2));
    println!("{}\n", part2(&lines1and2));
}

fn part1(lines: &Vec::<String>) -> i32 {
    let mut sum_of_ids = 0;
    for i in 0..lines.len() {
        let line_id = i as i32 + 1;
        let mut line_record: HashMap<String, i32> = HashMap::new();
        let line_of_balls = lines[i].split(' ').map(String::from).collect::<Vec<String>>()[2..].join(" ");
        let replaced_line = line_of_balls.replace(";", &",");
        let arr_of_balls = replaced_line.split(",").map(|s| s.trim().split(' ')).map(|spl| spl.collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        for arr in arr_of_balls {
            //println!("{}, {}", arr[0], arr[1]);
            let amount: i32 = arr[0].parse().unwrap();
            let color = arr[1];
            if !line_record.contains_key(color) {
                line_record.insert(color.to_owned(), amount);
            } else if line_record.get(color).unwrap() < &amount {
                *line_record.get_mut(color).unwrap() = amount;
            }
        }
        if  line_record.get("red").unwrap_or(&99) <= &12
            && line_record.get("green").unwrap_or(&99) <= &13 
            && line_record.get("blue").unwrap_or(&99) <= &14  {
            //println!("id: {}, red: {}, green: {}, blue: {}", line_id, line_record.get("red").unwrap(), line_record.get("green").unwrap(), line_record.get("blue").unwrap());
            sum_of_ids += line_id;
        }
    }
    return sum_of_ids;
}

fn part2(lines: &Vec::<String>) -> i32 {
    let mut sum_of_powers = 0;
    for i in 0..lines.len() {
        //let line_id = i as i32 + 1;
        let mut line_record: HashMap<String, i32> = HashMap::new();
        let line_of_balls = lines[i].split(' ').map(String::from).collect::<Vec<String>>()[2..].join(" ");
        let replaced_line = line_of_balls.replace(";", &",");
        let arr_of_balls = replaced_line.split(",").map(|s| s.trim().split(' ')).map(|spl| spl.collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
        for arr in arr_of_balls {
            //println!("{}, {}", arr[0], arr[1]);
            let amount: i32 = arr[0].parse().unwrap();
            let color = arr[1];
            if !line_record.contains_key(color) {
                line_record.insert(color.to_owned(), amount);
            } else if line_record.get(color).unwrap() < &amount {
                *line_record.get_mut(color).unwrap() = amount;
            }
        }
        let (red, blue, green) = (line_record.get("red").unwrap_or(&99), line_record.get("green").unwrap_or(&99), line_record.get("blue").unwrap_or(&99));
        //println!("id: {}, red: {}, green: {}, blue: {}, power: {}", line_id, red, blue, green, red * blue * green);

        sum_of_powers += red * green * blue;
    }
    return sum_of_powers;
}