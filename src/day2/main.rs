mod error;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::error::Result;

fn read_input(path: &str) -> Result<String> {
    let mut file = File::open(&path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

fn contains_char_count(line: &str, count: usize) -> bool {
    let mut counter = HashMap::new();

    for chr in line.chars() {
        let counter = counter.entry(chr).or_insert(0);
        *counter += 1;
    }

    counter.values().any(|&cnt| cnt == count)
}

fn main() -> Result<()> {
    // Part 1
    let contents = read_input("src/day2/input1.txt")?;

    let doubles = contents
        .lines()
        .filter(|line| contains_char_count(&line, 2))
        .count();

    let triples = contents
        .lines()
        .filter(|line| contains_char_count(&line, 3))
        .count();

    println!("Day 2 - Part 1: {}", doubles * triples);

    Ok(())
}

#[cfg(test)]
mod text {
    use super::*;

    #[test]
    fn contains_char_count_with_no_double() {
        assert_eq!(contains_char_count("abcdef", 2), false);
    }

    #[test]
    fn contains_char_count_with_a_double() {
        assert_eq!(contains_char_count("bababc", 2), true);
    }

    #[test]
    fn contains_char_count_with_multiple_doubles() {
        assert_eq!(contains_char_count("aabcdd", 2), true);
    }
}
