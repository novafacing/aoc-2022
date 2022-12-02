// For example, suppose you were given the following strategy guide:
//
// A Y
// B X
// C Z
// This strategy guide predicts and recommends the following:

// X is rock, Y is paper, Z is scissors.
//
// In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
// In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
// The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
// In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
//
// What would your total score be if everything goes exactly according to your strategy guide?

use std::io::{stdin, Read};

enum RPSResult {
    Win,
    Loss,
    Draw,
}

impl RPSResult {
    fn from_char(c: char) -> RPSResult {
        match c {
            'Z' => RPSResult::Win,
            'X' => RPSResult::Loss,
            'Y' => RPSResult::Draw,
            _ => panic!("Invalid result"),
        }
    }

    fn value(&self) -> u64 {
        match self {
            RPSResult::Win => 6,
            RPSResult::Loss => 0,
            RPSResult::Draw => 3,
        }
    }
}

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    fn from_char(c: char) -> Choice {
        match c {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissors,
            _ => panic!("Invalid choice"),
        }
    }

    fn value(&self) -> u64 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }

    fn opponent_play(result: &RPSResult, other: &Choice) -> Choice {
        match (result, other) {
            (RPSResult::Win, Choice::Rock) => Choice::Paper,
            (RPSResult::Win, Choice::Paper) => Choice::Scissors,
            (RPSResult::Win, Choice::Scissors) => Choice::Rock,
            (RPSResult::Loss, Choice::Rock) => Choice::Scissors,
            (RPSResult::Loss, Choice::Paper) => Choice::Rock,
            (RPSResult::Loss, Choice::Scissors) => Choice::Paper,
            (RPSResult::Draw, Choice::Rock) => Choice::Rock,
            (RPSResult::Draw, Choice::Paper) => Choice::Paper,
            (RPSResult::Draw, Choice::Scissors) => Choice::Scissors,
        }
    }
}

fn main() {
    // Read input from stdin
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    println!(
        "{}",
        input
            .lines()
            .map(|l| {
                let op = Choice::from_char(l.chars().nth(0).unwrap());
                let res = RPSResult::from_char(l.chars().nth(2).unwrap());
                let me = Choice::opponent_play(&res, &op);
                res.value() + me.value()
            })
            .sum::<u64>()
    );
}
