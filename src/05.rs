use setup_utils::*;
use std::ops::{Add, Range, Sub};
use std::path::Path;
use std::thread;

// Symbols to replace: 05 35 46 278755257 26829166

#[derive(Clone)]
struct RangeMap<T> {
    from: Range<T>,
    to: Range<T>,
}

#[allow(dead_code)]
impl<
        T: Sized + Copy + Clone + PartialOrd + std::fmt::Display + Add<Output = T> + Sub<Output = T>,
    > RangeMap<T>
{
    pub fn new(from_range: Range<T>, to_range: Range<T>) -> RangeMap<T> {
        return RangeMap::<T> {
            from: from_range,
            to: to_range,
        };
    }

    pub fn from_single(from_range: Range<T>) -> RangeMap<T> {
        // somewhat unnecessary
        return RangeMap::new(from_range.clone(), from_range);
    }

    pub fn contains_from(&self, value: &T) -> bool {
        return self.from.contains(value);
    }

    pub fn contains_to(&self, value: &T) -> bool {
        return self.to.contains(value);
    }

    pub fn map(&self, value: &T) -> T {
        if !self.contains_from(value) {
            panic!(
                "Value {value} not in {}..{}!",
                self.from.start, self.from.end
            );
        }
        return self.to.start + *value - self.from.start;
    }
}

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-example.txt"));
        let result = crate::part1(&lines);
        if result == 35 {
            Ok(())
        } else {
            Err(format!(
                "05: Bad result for Part 1 example, expected 35 got {}",
                result
            ))
        }
    }
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-example.txt"));
        let result = crate::part2(&lines);
        if result == 46 {
            Ok(())
        } else {
            Err(format!(
                "05: Bad result for Part 2 example, expected 46 got {}",
                result
            ))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/05-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (278755257, 26829166) => Ok(()),
            (_, 26829166) => Err(format!(
                "05: Bad result for Part 1, expected 278755257 got {}",
                result1
            )),
            (278755257, _) => Err(format!(
                "05: Bad result for Part 2, expected 26829166 got {}",
                result2
            )),
            (_, _) => Err(format!(
                "05: Bad result for Part 1 & 2, expected (278755257, 26829166) got ({}, {})",
                result1, result2
            )),
        }
    }
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/05-full.txt"));
    let lines = read_lines(Path::new("./inputs/05-example.txt"));

    println!("05-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("05-1-example.txt");
    println!("{}", part1(&lines));
    println!("{}\n", part2(&lines));
}

fn part1(lines: &Vec<String>) -> u64 {
    let seeds = lines[0].split(':').map(|s| s.trim()).collect::<Vec<_>>()[1]
        .split(' ')
        .map(|s| {
            s.parse::<u64>()
                .expect(&format!("Number was incorrect, number: {s}"))
        })
        .collect::<Vec<_>>();
    //println!("{seeds:?}");
    //first split categories
    let lines_categories = lines[2..].to_vec();
    let maps = lines_categories
        .split(|s| s == "")
        .map(|s| s.to_vec())
        .collect::<Vec<Vec<String>>>();
    let mut mappings: Vec<Vec<RangeMap<u64>>> = Vec::new();
    //println!("{maps:#?}");
    for map in maps {
        let _header = &map[0];
        let mapset = map[1..]
            .iter()
            .map(|s| {
                let split = s.split(' ').collect::<Vec<_>>();
                let dest_range_start = split[0].parse::<u64>().unwrap();
                let source_range_start = split[1].parse::<u64>().unwrap();
                let range_len = split[2].parse::<u64>().unwrap();
                //println!("{split:?}: {}..{}, {}..{}", source_range_start, source_range_start+range_len, dest_range_start, dest_range_start+range_len);
                RangeMap::new(
                    source_range_start..source_range_start + range_len + 1,
                    dest_range_start..dest_range_start + range_len + 1,
                )
            })
            .collect::<Vec<_>>();
        mappings.push(mapset);
    }
    let result = seeds
        .iter()
        .map(|seed| {
            let mut current_val = seed.clone();
            //print!("{current_val}");
            for mapset in &mappings {
                let mut found = false;
                let mut mapped: u64 = 0;
                for rangemap in mapset {
                    if rangemap.contains_from(&current_val) {
                        mapped = rangemap.map(&current_val);
                        found = true;
                        break;
                    }
                }
                if !found {
                    mapped = current_val;
                }
                current_val = mapped;
                //print!(" -> {current_val}");
            }
            //println!("");
            current_val
        })
        .collect::<Vec<_>>();

    //println!("{result:?}");

    return result.iter().min().unwrap().clone();
}

fn part2(lines: &Vec<String>) -> u64 {
    let seeds_unparsed = lines[0].split(':').map(|s| s.trim()).collect::<Vec<_>>()[1]
        .split(' ')
        .map(|s| {
            s.parse::<u64>()
                .expect(&format!("Number was incorrect, number: {s}"))
        })
        .collect::<Vec<_>>();
    let seed_ranges = seeds_unparsed
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1] + 1);
    //println!("{seeds:?}");
    //first split categories
    let lines_categories = lines[2..].to_vec();
    let maps = lines_categories
        .split(|s| s == "")
        .map(|s| s.to_vec())
        .collect::<Vec<Vec<String>>>();
    let mut mappings = Vec::new();
    //println!("{maps:#?}");
    for map in maps {
        let _header = &map[0];
        let mapset = map[1..]
            .iter()
            .map(|s| {
                let split = s.split(' ').collect::<Vec<_>>();
                let dest_range_start = split[0].parse::<u64>().unwrap();
                let source_range_start = split[1].parse::<u64>().unwrap();
                let range_len = split[2].parse::<u64>().unwrap();
                //println!("{split:?}: {}..{}, {}..{}", source_range_start, source_range_start+range_len, dest_range_start, dest_range_start+range_len);
                vec![dest_range_start, source_range_start, range_len]
            })
            .collect::<Vec<_>>();
        mappings.push(mapset);
    }
    let mut lowest = u64::MAX;
    let _ = thread::scope(|s| {
        let join_vec = seed_ranges
            .map(|seed_range| {
                s.spawn(|| {
                    let mut num_of_iters: i128 = 0;
                    return (seed_range
                        .map(|seed| {
                            let mut current_val = seed.clone();
                            //print!("{current_val}");
                            for mapset in &mappings {
                                let mut found = false;
                                let mut mapped: u64 = 0;
                                for rangearr in mapset {
                                    let rangemap = RangeMap::new(
                                        rangearr[1]..rangearr[1] + rangearr[2] + 1,
                                        rangearr[0]..rangearr[0] + rangearr[2] + 1,
                                    );
                                    if rangemap.contains_from(&current_val) {
                                        mapped = rangemap.map(&current_val);
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    mapped = current_val;
                                }
                                current_val = mapped;
                                //print!(" -> {current_val}");
                                num_of_iters += 1;
                            }
                            //println!("");
                            current_val
                        })
                        .min()
                        .unwrap(), num_of_iters);
                })
            })
            .collect::<Vec<_>>();
        let mut _i = 0;
        for jh in join_vec {
            let val = jh.join().unwrap();
            if val.0 < lowest {
                lowest = val.0;
            }
            //println!("Thread {i} returned, count of iterations performed: {}", val.1);
            _i += 1;
        }
    });
    //println!("{result:?}");
    return lowest;
    //return result.iter().min().unwrap().clone();
}
