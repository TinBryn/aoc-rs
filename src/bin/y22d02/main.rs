fn main() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("./input/y22/d02/input.txt")?;
    let input: Input = input.parse()?;
    problem1(&input);
    problem2(&input);

    Ok(())
}

fn problem1(_input: &Input) {
    let mut score = 0;
    for &(a, x) in &_input.pairs {
        let x = match x {
            Outcome::Lose => Play::Rock,
            Outcome::Draw => Play::Paper,
            Outcome::Win => Play::Scissors,
        };
        score += match (a, x) {
            (Play::Rock, Play::Rock) => 4,
            (Play::Rock, Play::Paper) => 8,
            (Play::Rock, Play::Scissors) => 3,
            (Play::Paper, Play::Rock) => 1,
            (Play::Paper, Play::Paper) => 5,
            (Play::Paper, Play::Scissors) => 9,
            (Play::Scissors, Play::Rock) => 7,
            (Play::Scissors, Play::Paper) => 2,
            (Play::Scissors, Play::Scissors) => 6,
        };
    }

    println!("score: {}", score);
}

fn problem2(_input: &Input) {
    let mut score = 0;
    for &(a, x) in &_input.pairs {
        score += match (a, x) {
            (Play::Rock, Outcome::Lose) => 3,
            (Play::Rock, Outcome::Draw) => 4,
            (Play::Rock, Outcome::Win) => 8,
            (Play::Paper, Outcome::Lose) => 1,
            (Play::Paper, Outcome::Draw) => 5,
            (Play::Paper, Outcome::Win) => 9,
            (Play::Scissors, Outcome::Lose) => 2,
            (Play::Scissors, Outcome::Draw) => 6,
            (Play::Scissors, Outcome::Win) => 7,
        };
    }

    println!("score: {}", score);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Play {
    Rock,     // A for rock, maybe X
    Paper,    // B for paper, maybe Y
    Scissors, // C for Scissors, maybe Z
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Lose, // X
    Draw, // Y
    Win,  // Z
}

pub struct Input {
    pairs: Vec<(Play, Outcome)>,
}

mod parsing {
    use std::str::FromStr;

    use crate::{Play, Outcome};
    use Play::*;
    use Outcome::*;
    #[derive(Debug)]
    pub enum InputParseError {
        MissingPart,
        Invalid,
    }

    impl FromStr for super::Input {
        type Err = InputParseError;
        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            let pairs = _s
                .lines()
                .map(|line| -> Result<(Play, Outcome), InputParseError> {
                    let line = line.as_bytes();
                    let (first, rest) = line.split_first().ok_or(InputParseError::MissingPart)?;
                    let last = *rest.last().ok_or(InputParseError::MissingPart)?;
                    let first = match first {
                        b'A' => Rock,
                        b'B' => Paper,
                        b'C' => Scissors,
                        _ => return Err(InputParseError::Invalid),
                    };
                    let last = match last {
                        b'X' => Lose,
                        b'Y' => Draw,
                        b'Z' => Win,
                        _ => return Err(InputParseError::Invalid),
                    };
                    Ok((first, last))
                })
                .collect::<Result<_, _>>()?;

            Ok(Self { pairs })
        }
    }
}

mod error {
    use crate::parsing::InputParseError;

    #[derive(Debug)]
    pub enum Error {
        IO(std::io::Error),
        Parse(InputParseError),
    }

    impl From<std::io::Error> for Error {
        fn from(err: std::io::Error) -> Self {
            Self::IO(err)
        }
    }

    impl From<InputParseError> for Error {
        fn from(err: InputParseError) -> Self {
            Self::Parse(err)
        }
    }
}
