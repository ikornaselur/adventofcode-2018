mod error;

use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

use crate::error::Result;

fn read_input(path: &str) -> Result<String> {
    let mut file = File::open(&path)?;

    let mut s = String::new();
    file.read_to_string(&mut s)?;

    Ok(s)
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    left: usize,
    right: usize,
    width: usize,
    height: usize,
}

impl From<&str> for Claim {
    fn from(string: &str) -> Claim {
        let re = Regex::new(
            r"#(?P<index>\d+) @ (?P<left>\d+),(?P<right>\d+): (?P<width>\d+)x(?P<height>\d+)",
        )
        .unwrap();
        let caps = re.captures(string).unwrap();
        Claim {
            id: caps["index"].parse().unwrap(),
            left: caps["left"].parse().unwrap(),
            right: caps["right"].parse().unwrap(),
            width: caps["width"].parse().unwrap(),
            height: caps["height"].parse().unwrap(),
        }
    }
}

fn claim_fabric(_fabric: &mut Vec<Vec<i32>>, _claim: &str) {
    ()
}

fn main() -> Result<()> {
    // Part 1
    let _contents = read_input("src/day3/input1.txt")?;

    let mut fabric: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];

    claim_fabric(&mut fabric, "foo");

    println!("Day 3 - Part 1: {}", 0);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_from_str_to_claim() {
        let claim_str = "#1 @ 1,3: 5x4";
        let claim = Claim::from(claim_str);
        println!("{:?}", claim);

        assert_eq!(
            claim,
            Claim {
                id: 1,
                left: 1,
                right: 3,
                width: 5,
                height: 4
            }
        );
    }

    #[test]
    fn claim_fabric_marks_single_claim() {
        let mut fabric: Vec<Vec<i32>> = vec![vec![0; 8]; 8];

        claim_fabric(&mut fabric, "#1 @ 1,3: 4x4");

        assert_eq!(
            fabric,
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
            ]
        );
    }

    #[test]
    fn claim_fabric_marks_multiple_claims() {
        let mut fabric: Vec<Vec<i32>> = vec![vec![0; 8]; 8];

        claim_fabric(&mut fabric, "#1 @ 1,3: 4x4");

        assert_eq!(
            fabric,
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
            ]
        );

        claim_fabric(&mut fabric, "#2 @ 3,1: 4x4");
        assert_eq!(
            fabric,
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 1, 1, 1, 1, 0],
                vec![0, 1, 1, 2, 2, 1, 1, 0],
                vec![0, 1, 1, 2, 2, 1, 1, 0],
                vec![0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 1, 1, 1, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
            ]
        );

        claim_fabric(&mut fabric, "#3 @ 5,5: 2x2");
        assert_eq!(
            fabric,
            vec![
                vec![0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 1, 1, 1, 1, 0],
                vec![0, 1, 1, 2, 2, 1, 1, 0],
                vec![0, 1, 1, 2, 2, 1, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 1, 0],
                vec![0, 1, 1, 1, 1, 1, 1, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
            ]
        );
    }
}
