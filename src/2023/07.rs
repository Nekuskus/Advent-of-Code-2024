use setup_utils::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;

#[macro_use]
extern crate lazy_static;

// Symbols to replace: 07 6440 5905 246795406 249356515

#[cfg(test)]
mod tests {
    use setup_utils::read_lines;
    use std::path::Path;

    #[test]
    fn part1() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/07-example.txt"));
        let result = crate::part1(&lines);
        if result == 6440 {
            Ok(())
        } else {
            Err(format!(
                "07: Bad result for Part 1 example, expected 6440 got {}",
                result
            ))
        }
    }
    
    #[test]
    fn part2() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/07-example.txt"));
        let result = crate::part2(&lines);
        if result == 5905 {
            Ok(())
        } else {
            Err(format!("07: Bad result for Part 2 example, expected 5905 got {}", result))
        }
    }

    #[test]
    fn full() -> Result<(), String> {
        let lines = read_lines(Path::new("./inputs/07-full.txt"));
        let result1 = crate::part1(&lines);
        let result2 = crate::part2(&lines);

        match (result1, result2) {
            (246795406, 249356515) => Ok(()),
            (_, 249356515) => Err(format!("07: Bad result for Part 1, expected 246795406 got {}", result1)),
            (246795406, _) => Err(format!("07: Bad result for Part 2, expected 249356515 got {}", result2)),
            (_, _) => Err(format!("07: Bad result for Part 1 & 2, expected (246795406, 249356515) got ({}, {})", result1, result2))
        }
    }
    
}

fn main() {
    let linesfull = read_lines(Path::new("./inputs/07-full.txt"));
    let lines1 = read_lines(Path::new("./inputs/07-example.txt"));

    println!("07-full.txt");
    println!("{}", part1(&linesfull));
    println!("{}\n", part2(&linesfull));

    println!("07-example.txt");
    println!("{}", part1(&lines1));
    println!("{}\n", part2(&lines1));
}

#[derive(Debug)]
struct Hand {
    r#type: String,
    cards: String,
    bid: i32,
}

lazy_static! {
    static ref VALUES: HashMap<char, i8> = {
        HashMap::from([
            ('2', 0),
            ('3', 1),
            ('4', 2),
            ('5', 3),
            ('6', 4),
            ('7', 5),
            ('8', 6),
            ('9', 7),
            ('T', 8),
            ('J', 9),
            ('Q', 10),
            ('K', 11),
            ('A', 12),
        ])
    };
    static ref VALUES_JOKER: HashMap<char, i8> = {
        HashMap::from([
            ('2', 0),
            ('3', 1),
            ('4', 2),
            ('5', 3),
            ('6', 4),
            ('7', 5),
            ('8', 6),
            ('9', 7),
            ('T', 8),
            ('Q', 10),
            ('K', 11),
            ('A', 12),
            ('J', -1),
        ])
    };
    static ref TYPES: HashMap<String, i8> = {
        HashMap::from([
            ("high".to_string(), 0),
            ("onepair".to_string(), 1),
            ("twopair".to_string(), 2),
            ("threekind".to_string(), 3),
            ("fullhouse".to_string(), 4),
            ("fourkind".to_string(), 5),
            ("fivekind".to_string(), 6),
        ])
    };
}

#[inline(always)]
fn calculate_hand(hand: String, bid: i32) -> Hand {
    let mut map: HashMap<char, i8> = HashMap::new();
    let mut pairs = 0;
    let mut r#type = "high";
    for c in hand.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    for val in map.values() {
        match val {
            2 => pairs += 1,
            3 => r#type = "threekind",
            4 => r#type = "fourkind",
            5 => r#type = "fivekind",
            _ => (),
        }
    }

    if pairs == 1 && r#type == "threekind" {
        r#type = "fullhouse";
    } else if pairs == 1 {
        r#type = "onepair";
    } else if pairs == 2 {
        r#type = "twopair";
    }

    return Hand {
        r#type: r#type.to_string(),
        cards: hand.to_string(),
        bid: bid,
    };
}

#[inline(always)]
fn calculate_hand_jokers(hand: String, bid: i32) -> Hand {
    let mut map: HashMap<char, i8> = HashMap::new();
    let mut pairs = 0;
    let mut threekinds = 0;
    let mut r#type = "high";
    for c in hand.chars() {
        *map.entry(c).or_insert(0) += 1;
    }

    let jokers = map.get(&'J').unwrap_or(&0);
    let mut jokers_for_pairs = jokers.clone();
    for (key, val) in &map {
        match if key != &'J' { val + jokers } else { *val } {
            2 => if val + jokers_for_pairs == 2 && key != &'J' {pairs += 1; if jokers_for_pairs > 0 {jokers_for_pairs -= 1}},
            3 => if TYPES.get(r#type).unwrap() <= TYPES.get("threekind").unwrap() {r#type = "threekind"; threekinds += 1;},
            4 => if TYPES.get(r#type).unwrap() <= TYPES.get("fourkind").unwrap() {r#type = "fourkind"},
            5 => if TYPES.get(r#type).unwrap() <= TYPES.get("fivekind").unwrap() {r#type = "fivekind"},
            _ => (),
        }
    }

    if pairs == 1 && r#type == "threekind" && jokers != &1 {
        if TYPES.get(r#type).unwrap() < TYPES.get("fullhouse").unwrap() {r#type = "fullhouse"};
    } else if pairs == 1 {
        if TYPES.get(r#type).unwrap() < TYPES.get("onepair").unwrap() {r#type = "onepair"};
    } else if pairs == 2 {
        if TYPES.get(r#type).unwrap() < TYPES.get("twopair").unwrap() {r#type = "twopair"};
    }
    
    if threekinds == 2 && jokers == &1 {
        r#type = "fullhouse";
    }

    return Hand {
        r#type: r#type.to_string(),
        cards: hand.to_string(),
        bid: bid,
    };
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut hands = lines
        .iter()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<_>>();
            calculate_hand(split[0].to_string(), split[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<Hand>>();

    hands.sort_unstable_by(|h1, h2| {
        let mut c1 = TYPES
            .get(&h1.r#type)
            .unwrap()
            .cmp(TYPES.get(&h2.r#type).unwrap());
        if c1 != Ordering::Equal {
            return c1;
        }

        let mut i = 0;
        while c1 == Ordering::Equal && i < 5 {
            c1 = VALUES
                .get(&(h1.cards.chars().nth(i).unwrap()))
                .unwrap()
                .cmp(VALUES.get(&(h2.cards.chars().nth(i).unwrap())).unwrap());
            i += 1;
        }
        return c1;
    });

    let tup = hands.iter().fold((0, 0), |(acc, index), value| {
        return (acc + value.bid * (index + 1), index + 1);
    });
    return tup.0;
}

fn part2(lines: &Vec::<String>) -> i32 {
    let mut hands = lines
        .iter()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<_>>();
            calculate_hand_jokers(split[0].to_string(), split[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<Hand>>();

    hands.sort_unstable_by(|h1, h2| {
        let mut c1 = TYPES
            .get(&h1.r#type)
            .unwrap()
            .cmp(TYPES.get(&h2.r#type).unwrap());
        if c1 != Ordering::Equal {
            return c1;
        }

        let mut i = 0;
        while c1 == Ordering::Equal && i < 5 {
            c1 = VALUES_JOKER
                .get(&(h1.cards.chars().nth(i).unwrap()))
                .unwrap()
                .cmp(VALUES_JOKER.get(&(h2.cards.chars().nth(i).unwrap())).unwrap());
            i += 1;
        }
        return c1;
    });

    for hand in &hands {
        println!("{:?}", hand)
    }

    let tup = hands.iter().fold((0, 0), |(acc, index), value| {
        return (acc + value.bid * (index + 1), index + 1);
    });
    return tup.0;
}