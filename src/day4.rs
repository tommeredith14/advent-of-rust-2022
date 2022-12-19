use crate::helpers::read_lines;
use std::str::FromStr;

pub fn test_day4() {
    println!("Running day 4!");
    run();
}

struct Range {
    upper: u32,
    lower: u32
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<&str> = s.split('-').collect();
        let lower = nums[0].parse().unwrap();
        let upper = nums[1].parse().unwrap();
        Ok(Self {
            upper,
            lower,
        })
    }
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }
}

struct ElfPair {
    first: Range,
    second: Range,
}

impl FromStr for ElfPair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves: Vec<&str> = s.split(',').collect();
        let first = elves[0].parse().unwrap();
        let second = elves[1].parse().unwrap();
        Ok(Self {
            first,
            second,
        })
    }
}

impl ElfPair {
    fn redundant(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    fn overlap(&self) -> bool {
        self.first.upper >= self.second.lower && self.first.lower <= self.second.upper
    }
}


fn run() {
    let Ok(lines) = read_lines("./data/day4.txt") else {
        println!("Error! couldn't read file");
        return;
    };
    let lines = lines.filter_map(|l| l.ok());

    let total = lines.map(|s| s.parse::<ElfPair>())
                     .filter_map(|ep| ep.ok())
                     .filter(|ep| ep.redundant())
                     .count();

    println!("Redundant pairs: {}", total);

    let Ok(lines) = read_lines("./data/day4.txt") else {
        println!("Error! couldn't read file");
        return;
    };
    let lines = lines.filter_map(|l| l.ok());

    let total = lines.map(|s| s.parse::<ElfPair>())
                     .filter_map(|ep| ep.ok())
                     .filter(|ep| ep.overlap())
                     .count();

    println!("Overlapping pairs: {}", total)

}

