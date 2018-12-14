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

/// Checks if a strings has the exact count of any character
///
/// This the string abbbcccd would return true for 1 or 3, but false for every thing else, since it
/// has 1 a (and 1d) or 3 b (and 3 c).
fn contains_char_count(line: &str, count: usize) -> bool {
    let mut counter = HashMap::new();

    for chr in line.chars() {
        let counter = counter.entry(chr).or_insert(0);
        *counter += 1;
    }

    counter.values().any(|&cnt| cnt == count)
}

/// Number of characters that differ between two stringso
fn char_diff_count(first: &str, second: &str) -> usize {
    first
        .chars()
        .zip(second.chars())
        .filter(|(a, b)| a != b)
        .count()
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

    // Part 2
    // Input remains the same

    let mut result = String::new();
    'outer: for (idx, line) in contents.lines().enumerate() {
        for second_line in contents.lines().skip(idx + 1) {
            if char_diff_count(line, second_line) == 1 {
                result = line
                    .chars()
                    .zip(second_line.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .collect();
                break 'outer;
            }
        }
    }
    println!("Day 2 - Part 2: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
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

    #[test]
    fn char_diff_count_matches_same_strings() {
        assert_eq!(char_diff_count("abcd", "abcd"), 0);
    }

    #[test]
    fn char_diff_count_counts_different_characters() {
        assert_eq!(char_diff_count("abcd", "axcd"), 1);
    }

    #[test]
    fn char_diff_count_counts_different_characters_independently() {
        assert_eq!(char_diff_count("abcd", "acbd"), 2);
    }
}
