use itertools::Itertools;
// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;

use crate::helpers::read_lines;

pub fn test_day1() {
    println!("Running day 1!");
    run();
}

fn run() {
    let Ok(lines) = read_lines("./data/day1.txt") else {
        println!("Error! couldn't read file");
        return;
    };

    // Extract the lines into groupings by elf
    let elves = &lines
        .filter_map(|l| l.ok())
        .into_iter()
        .group_by(|s| !s.is_empty());
    let elves = elves // TODO: How to combine these two?
        .into_iter()
        .filter(|g| g.0)
        .map(|g| g.1);

    // Get the total calories of each elf
    let elves = elves.map(|g| g.filter_map(|s| s.parse::<i32>().ok()).sum::<i32>());

    // Get te top 3 elves
    let mut top_elves = elves
        .sorted_by(|a, b| b.cmp(a)) // reverse sort
        .take(3)
        .peekable();

    let Some(max_elf) = top_elves.peek().cloned() else {
        println!("No max elf!");
        return;
    };

    println!("Max elf: {}", max_elf);

    println!("Top 3: {}", top_elves.sum::<i32>());
}

// // The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
// // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }
