use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn fuel_for_module(mass: u32) -> u32 {
    (mass / 3) - 2
}

pub fn run() -> u32 {
    let file = File::open("day1_input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .map(fuel_for_module)
        .sum()
}
