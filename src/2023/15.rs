use setup_utils::*;
use std::{path::Path, collections::HashMap};


// Symbols to replace: 15 1320 145 514639 279470


#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/15-example.txt"));
        let result = crate::part1(&lines);
        if result == 1320 {
            Ok(())
        } else {
            Err(format!("15: Bad result for Part 1 example, expected 1320 got {}", result))
        }
    }
    
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/15-example.txt"));
        let result = crate::part2(&lines);
        if result == 145 {
            Ok(())
        } else {
            Err(format!("15: Bad result for Part 2 example, expected 145 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/15-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (514639, 279470) => Ok(()),
            (_, 279470) => Err(format!("15: Bad result for Part 1, expected 514639 got {}", result1)),
            (514639, _) => Err(format!("15: Bad result for Part 2, expected 279470 got {}", result2)),
            (_, _) => Err(format!("15: Bad result for Part 1 & 2, expected (514639, 279470) got ({}, {})", result1, result2))
        }
    }
    
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/15-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/15-example.txt"));

    println!("15-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));
    
    println!("15-1-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

fn part1(lines: &Vec<String>) -> u32 {
    // Ignore newlines, though there aren't any in the input provided
    let line = lines.join("");
    let split = line.split(",");

    split.map(|s| hash(s)).sum()
}

fn part2(lines: &Vec<String>) -> u32 {
    // <hash, (Label, focal_length)[]>
    let mut boxes: HashMap<u32, Vec<(String, u32)>> = HashMap::new();

    let line = lines.join("");
    let split = line.split(",");

    for s in split {
        // after next is = or -, 0 or 1 left in chars
        let mut chars = s.chars();
        let mut label = chars.next().unwrap().to_string();
        let mut next = chars.next().unwrap();
        while next != '=' && next != '-' {
            label.push(next);
            next = chars.next().unwrap();
        }
        let hash = hash(&label);
        let oper = next;
        
        match oper {
            '-' => {
                let cur_box = match boxes.get_mut(&hash) {
                    Some(box_found) => box_found,
                    None => { continue; }
                };
                let pos = match cur_box.iter().position(|(label_inner, _)| label_inner == &label) {
                    Some(n) => n,
                    None => { continue; }
                };
                cur_box.remove(pos);
            }
            '=' => {
                let focal_length = chars.next().unwrap().to_string().parse::<u32>().unwrap();
                let cur_box = match boxes.get_mut(&hash) {
                    Some(box_found) => box_found,
                    None => { boxes.insert(hash, vec![(label, focal_length)]); continue; }
                };
                match cur_box.iter().position(|(label_inner, _focal_length)| label_inner == &label) {
                    Some(n) => { cur_box[n].1 = focal_length},
                    None => { cur_box.push((label, focal_length)); }
                };
            }
            _ => panic!("Invalid value in oper: {oper}")
        }
    }
    //println!("{boxes:?}");
    boxes.iter().map(|(hash, contents)| contents.iter().enumerate().map(|(index, (_label, focal_length))| (hash + 1) * (index as u32 + 1) * focal_length).sum::<u32>()).sum()
}
