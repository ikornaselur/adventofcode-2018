mod error;

use std::fs::File;
use std::io::prelude::*;

use crate::error::Result;

fn read_input(path: &str) -> Result<String> {
    let mut file = File::open(&path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

fn main() -> Result<()> {
    // Part 1
    let contents = read_input("src/day1/input1.txt")?;

    let result: i32 = contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum();
    println!("Day 1 - Part 1: {}", result);

    Ok(())
}
