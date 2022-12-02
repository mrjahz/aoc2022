use std::{fs, string::FromUtf8Error};

mod day1;
mod day2;

fn parse_full_input(path: &str) -> Result<String, FromUtf8Error> {
    String::from_utf8(fs::read(path).unwrap())
}
fn main() {
    println!("Hello, AOC2022 !");
    println!("Day1: {:?}", day1::get_answer());

    let day2_input = parse_full_input("assets/day2_puzzle1_input.txt").unwrap();
    println!(
        "Day2: {:?}, {:?}",
        day2::get_part1_answer(&day2_input),
        day2::get_part2_answer(&day2_input)
    );
}
