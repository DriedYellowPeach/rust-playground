use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() {
    println!("Advent of Code Day 2");
    println!(">>> {}", calculate_game_point("aoc/input/day2_input.txt"));
    println!("Advent of Code Day 2 Part 2");
    println!(">>> {}", calculate_game_point_part2("aoc/input/day2_input.txt"));
}

#[derive(Clone, Copy)]
enum Card {
    Rock,
    Paper,
    Scissors,
}

impl Card {
    fn get_card_point(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn fight(&self, other: &Self) -> i32 {
        match (self, other) {
            (Card::Rock, Card::Rock) => 3,
            (Card::Rock, Card::Scissors) => 6,
            (Card::Rock, Card::Paper) => 0,

            (Card::Paper, Card::Paper) => 3,
            (Card::Paper, Card::Rock) => 6,
            (Card::Paper, Card::Scissors) => 0,

            (Card::Scissors, Card::Scissors) => 3,
            (Card::Scissors, Card::Paper) => 6,
            (Card::Scissors, Card::Rock) => 0,
        }
    }

    fn fail(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn draw(&self) -> Self {
        *self
    }

    fn counter(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub fn get_turn_point(&self, other: &Self) -> i32 {
        self.get_card_point() + self.fight(other)
    }
}

enum TurnResult {
    Win,
    Draw,
    Lose,
}

impl TurnResult {
    fn should_play(&self, other: &Card) -> Card {
        match self {
            Self::Win => other.fail(),
            Self::Draw => other.draw(),
            Self::Lose => other.counter(),
        }
    }

    fn get_result_point(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Lose => 0,
        }
    }
}

use std::convert::From;

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Card::Rock,
            'B' | 'Y' => Card::Paper,
            'C' | 'Z' => Card::Scissors,
            _ => {
                panic!("Input format error: invalid card type");
            }
        }
    }
}

impl From<char> for TurnResult {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => {
                panic!("Input format error: invalid card type");
            }
        }
    }
}

fn calculate_game_point(path: &str) -> i32 {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {}", e);
            return -1;
        }
    };
    let mut acc = 0;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("Failed to read line: {}", e);
                return -1;
            }
        };

        let fight = line.split_whitespace().collect::<Vec<_>>();
        if fight.len() != 2 {
            println!("Input format error: can't split");
            return -1;
        }

        let (rival, me) = (fight[0], fight[1]);
        if rival.len() != 1 && me.len() != 1 {
            println!("Input format error: invalid card");
            return -1;
        }

        acc += Card::from(me.chars().next().unwrap())
            .get_turn_point(&Card::from(rival.chars().next().unwrap()));
    }
    acc
}

fn calculate_game_point_part2(path: &str) -> i32 {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to open file: {}", e);
            return -1;
        }
    };
    let mut acc = 0;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("Failed to read line: {}", e);
                return -1;
            }
        };

        let fight = line.split_whitespace().collect::<Vec<_>>();
        if fight.len() != 2 {
            println!("Input format error: can't split");
            return -1;
        }

        let (rival, result) = (fight[0], fight[1]);
        if rival.len() != 1 && result.len() != 1 {
            println!("Input format error: invalid card");
            return -1;
        }

        let (rival, result) = (
            Card::from(rival.chars().next().unwrap()),
            TurnResult::from(result.chars().next().unwrap()),
        );
        let me = result.should_play(&rival);
        acc += result.get_result_point() + me.get_card_point();

        // acc += TurnResult::from(result.chars.next()).
    }
    acc
}

#[test]
fn test_calculate_game_point() {
    assert_eq!(calculate_game_point("input/day2_input_example.txt"), 15);
    assert_eq!(calculate_game_point_part2("input/day2_input_example.txt"), 12);
}
