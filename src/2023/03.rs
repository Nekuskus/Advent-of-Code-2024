use setup_utils::{len, read_lines, pad_lines};
use std::path::Path;

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/03-example.txt"));
        let result = crate::part1(&lines);
        if result == 4361 {
            Ok(())
        } else {
            Err(format!(
                "03: Bad result for Part 1 example, expected 4361 got {}",
                result
            ))
        }
    }

    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/03-example.txt"));
        let result = crate::part2(&lines);
        if result == 467835 {
            Ok(())
        } else {
            Err(format!(
                "03: Bad result for Part 2 example, expected 467835 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/03-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);
        match (result1, result2) {
            (525181, 84289137) => Ok(()),
            (_, 84289137) => Err(format!(
                "03: Bad result for Part 1, expected 525181 got {}",
                result1
            )),
            (525181, _) => Err(format!(
                "03: Bad result for Part 2, expected 84289137 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "03: Bad result for Part 1 & 2, expected (525181, 84289137) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/03-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/03-example.txt"));

    println!("03-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("03-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
}


fn part1(lines: &Vec<String>) -> i32 {
    let mut sum_of_nums = 0;

    let mut cur_parsing = false;
    let mut cur_num = String::from("");
    let mut cur_is_valid = false;

    for y in 0..len!(lines) {
        for x in 0..len!(lines[y]) {
            if x == 0 {
                // handle line break while parsing number! as in, stop parsing and dump the result
                if cur_parsing {
                    let number = cur_num.parse::<i32>().expect(&format!(
                        "Something horrible happened and somehow instead of numbers value was {}",
                        cur_num
                    ));
                    if cur_is_valid {
                        sum_of_nums += number;
                    }
                }
                cur_parsing = false;
                cur_is_valid = false;
                cur_num = String::from("");
            }

            let line = lines[y].chars().collect::<Vec<char>>();
            let c = line[x];

            if c.is_digit(10) {
                cur_parsing = true;
                cur_num += &c.to_string();

                let rangey = 0..len!(lines) as i32;
                let rangex = 0..len!(lines[y]) as i32;

                if rangey.contains(&(y as i32 - 1)) {
                    let line_prev = lines[y - 1].chars().collect::<Vec<char>>();
                    if rangex.contains(&(x as i32 - 1)) {
                        if !line_prev[x - 1].is_digit(10) && line_prev[x - 1] != '.' {
                            cur_is_valid = true;
                        }
                    }
                    if rangex.contains(&(x as i32 + 1)) {
                        if !line_prev[x + 1].is_digit(10) && line_prev[x + 1] != '.' {
                            cur_is_valid = true;
                        }
                    }
                    if !line_prev[x].is_digit(10) && line_prev[x] != '.' {
                        cur_is_valid = true;
                    }
                }
                if rangey.contains(&(y as i32 + 1)) {
                    let line_next = lines[y + 1].chars().collect::<Vec<char>>();
                    if rangex.contains(&(x as i32 - 1)) {
                        if !line_next[x - 1].is_digit(10) && line_next[x - 1] != '.' {
                            cur_is_valid = true;
                        }
                    }
                    if rangex.contains(&(x as i32 + 1)) {
                        if !line_next[x + 1].is_digit(10) && line_next[x + 1] != '.' {
                            cur_is_valid = true;
                        }
                    }
                    if !line_next[x].is_digit(10) && line_next[x] != '.' {
                        cur_is_valid = true;
                    }
                }
                if rangex.contains(&(x as i32 - 1)) {
                    if !line[x - 1].is_digit(10) && line[x - 1] != '.' {
                        cur_is_valid = true;
                    }
                }
                if rangex.contains(&(x as i32 + 1)) {
                    if !line[x + 1].is_digit(10) && line[x + 1] != '.' {
                        cur_is_valid = true;
                    }
                }
            } else {
                // c == '.' or c is a symbol, doesn't matter because symbols are handled only as adjacent to the previous case
                if cur_parsing {
                    let number = cur_num.parse::<i32>().expect(&format!(
                        "Something horrible happened and somehow instead of numbers value was {}",
                        cur_num
                    ));
                    if cur_is_valid {
                        sum_of_nums += number;
                        //println!("{}", number);
                    }
                }
                cur_parsing = false;
                cur_is_valid = false;
                cur_num = String::from("");
            }
        }
    }
    return sum_of_nums;
}

fn parse_num_from(line: &Vec<char>, x: usize) -> i32 {
    let mut cur_num = String::from("");
    let mut found_start = x;
    //find num start

    while line[found_start].is_digit(10) && found_start != 0 {
        found_start -= 1;
    }

    if !line[found_start].is_digit(10) {
        found_start += 1;
    }
    //println!("{}", &line[found_start]);

    while found_start != len!(line) && line[found_start].is_digit(10) {
        cur_num += &line[found_start].to_string();
        found_start += 1;
    }

    //println!("{}", cur_num);

    return cur_num.parse::<i32>().expect(&format!(
        "Something horrible happened and somehow instead of numbers value was {}",
        cur_num
    ));
}

fn part2(lines: &Vec<String>) -> i32 {
    let padded = pad_lines(lines);
    let mut sum_of_nums = 0;
    let mut lastnum = -1;

    for y in 0..len!(padded) {
        for x in 0..len!(padded[y]) {
            let line = padded[y].chars().collect::<Vec<char>>();
            let c = line[x];

            let mut found_numbers = Vec::new();

            if c == '*' {
                let line_prev = padded[y - 1].chars().collect::<Vec<char>>();

                if line_prev[x - 1].is_digit(10) {
                    let parsed = parse_num_from(&line_prev, x - 1);
                    if parsed != lastnum {
                        found_numbers.push(parsed);
                    }
                    lastnum = parsed;
                }
                if line_prev[x + 1].is_digit(10) {
                    let parsed = parse_num_from(&line_prev, x + 1);
                    if parsed != lastnum {
                        found_numbers.push(parsed);
                    }
                    lastnum = parsed;
                }

                if line_prev[x].is_digit(10) {
                    let parsed = parse_num_from(&line_prev, x);
                    if parsed != lastnum {
                        found_numbers.push(parsed);
                    }
                    lastnum = parsed;
                }

                let line_next = padded[y + 1].chars().collect::<Vec<char>>();

                if line_next[x - 1].is_digit(10) {
                    let parsed = parse_num_from(&line_next, x - 1);
                    if parsed != lastnum {
                        found_numbers.push(parsed);
                    }
                    lastnum = parsed;
                }

                if line_next[x + 1].is_digit(10) {
                    let parsed = parse_num_from(&line_next, x + 1);
                    if parsed != lastnum {
                        found_numbers.push(parsed);
                    }
                    lastnum = parsed;
                }

                if line_next[x].is_digit(10) {
                    let parsed = parse_num_from(&line_next, x);
                    if parsed != lastnum {
                        found_numbers.push(parsed);
                    }
                    lastnum = parsed;
                }
                if line[x - 1].is_digit(10) {
                    let parsed = parse_num_from(&line, x - 1);
                    if parsed != lastnum {
                        found_numbers.push(parsed);
                    }
                    lastnum = parsed;
                }

                if line[x + 1].is_digit(10) {
                    let parsed = parse_num_from(&line, x + 1);
                    if parsed != lastnum {
                        found_numbers.push(parsed);
                    }
                    lastnum = parsed;
                }

                //println!("cursum: {}, oper1: {}, oper2: {}",sum_of_nums, oper1, oper2);
                if len!(found_numbers) == 2 {
                    //println!("in if -- cursum: {}, oper1: {}, oper2: {}",sum_of_nums, oper1, oper2);
                    sum_of_nums += found_numbers[0] * found_numbers[1];
                }
            }
        }
    }
    return sum_of_nums;
}
