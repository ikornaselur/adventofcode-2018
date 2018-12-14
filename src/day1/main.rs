mod error;

use std::collections::HashSet;
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

    let parsed: Vec<i32> = contents
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    let result: i32 = parsed.iter().sum();
    println!("Day 1 - Part 1: {}", result);

    // Part 2
    // Input remains the same
    let mut sum: i32 = 0;
    let mut seen = HashSet::new();

    'outer: loop {
        for value in parsed.iter() {
            seen.insert(sum);

            sum += value;
            if seen.contains(&sum) {
                break 'outer;
            }
        }
    }
    println!("Day 1 - Part 2: {}", sum);

    Ok(())
}
