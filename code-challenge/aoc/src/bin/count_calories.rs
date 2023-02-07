use std::env;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Advent of Code, Day 1 2022");
    println!("current dir {:?}", env::current_dir().unwrap());
    println!("{}", calculate_calories("aoc/input/day1_input.txt"));
}

/// the calories can't be negative, if negative, means something wrong when reading the input file
fn calculate_calories(path: &str) -> i64 {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            println!("open faild: {}", e);
            return -1;
        }
    };
    let mut max_cal = 0;
    let mut acc = 0;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("{}", e);
                return -1;
            }
        };

        if line.trim().is_empty() {
            max_cal = std::cmp::max(max_cal, acc);
            acc = 0;
            continue;
        }

        acc += match line.parse::<i64>() {
            Ok(i) => i,
            Err(e) => {
                println!("line: {} parse error: {}", line, e);
                return -1;
            }
        }
    }

    // handle the last acc
    if acc != 0 {
        max_cal = std::cmp::max(max_cal, acc);
    }

    max_cal
}

#[test]
fn test_cal_calories() {
    println!("current dir {:?}", env::current_dir().unwrap());
    assert_eq!(calculate_calories("input/day1_input.txt"), 24000);
}
