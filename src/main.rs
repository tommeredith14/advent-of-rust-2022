#![allow(unused_imports)]

use crate::day1::test_day1;
use crate::day2::test_day2;
use crate::day3::test_day3;
use crate::day4::test_day4;

pub mod helpers;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

fn main() {
    println!("Hello, world!");
    test_day4();
}
