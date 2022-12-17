use itertools::{self, Itertools};
use std::collections::HashMap;

use crate::helpers::read_lines;

pub fn test_day3() {
    println!("Running day 3!");
    run();
}

fn common_item(s: &str) -> char {
    let (bag1, bag2) = s.split_at(s.len() / 2);

    let mut present = HashMap::new();
    for c in bag1.chars() {
        present.insert(c, true);
    }

    for c in bag2.chars() {
        if present.contains_key(&c) {
            return c;
        }
    }
    '1'
}

fn common_item3(v: Vec<String>) -> char {
    let mut present = HashMap::new();
    for c in v[0].chars() {
        present.insert(c, 1);
    }
    for c in v[1].chars() {
        if present.contains_key(&c) {
            present.insert(c, 2); //insert(c, 1);
        }
    }
    for c in v[2].chars() {
        if Some(&2) == present.get(&c) {
            return c;
        }
    }

    '1'
}

fn item_priority(c: char) -> i32 {
    match c {
        'a'..='z' => c as i32 - 'a' as i32 + 1,
        'A'..='Z' => c as i32 - 'A' as i32 + 27,
        _ => 1,
    }
}

fn run() {
    let Ok(lines) = read_lines("./data/day3.txt") else {
        println!("Error! couldn't read file");
        return;
    };
    let lines = lines.filter_map(|l| l.ok());

    let total = lines
        .map(|s| common_item(&s))
        .map(item_priority)
        .sum::<i32>();

    println!("Total: {}", total);

    let Ok(lines) = read_lines("./data/day3.txt") else {
                        println!("Error! couldn't read file");
                        return;
                    };
    let lines = lines.filter_map(|l| l.ok());

    let total = lines
        .chunks(3)
        .into_iter()
        .map(|chunk| common_item3(chunk.collect_vec()))
        .map(item_priority)
        .sum::<i32>();

    println!("Total: {}", total)
}
