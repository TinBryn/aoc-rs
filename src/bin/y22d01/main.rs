fn main() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("./input/y22/d01/input.txt")?;
    let input: Input = input.parse()?;
    problem1(&input);
    problem2(&input);

    Ok(())
}

fn problem1(input: &Input) {
    {
        let totals = totals(input);

        let max = totals.iter().max();

        if let Some(max) = max {
            println!("max is {}", max)
        }
    }
}

fn totals(input: &Input) -> Vec<i64> {
    let totals: Vec<i64> = input
        .elves_food
        .iter()
        .map(|elf| elf.iter().sum())
        .collect();
    totals
}

fn problem2(input: &Input) {
    {
        let mut totals: Vec<i64> = totals(input);
        totals.sort();
        totals.reverse();
        let top3: i64 = totals.iter().take(3).sum();
        println!("total of top three is {}", top3);
    }
}

use input::Input;
mod input {
    pub struct Input {
        pub elves_food: Vec<Vec<i64>>,
    }
}

mod parsing {
    use std::str::FromStr;
    #[derive(Debug)]
    pub enum InputParseError {
        NonNum,
    }

    impl FromStr for super::Input {
        type Err = InputParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut elves_food = vec![];
            let mut food = vec![];
            for line in s.lines() {
                let line = line.trim();
                if line.is_empty() {
                    elves_food.push(std::mem::take(&mut food));
                } else {
                    let n = line.parse().map_err(|_| InputParseError::NonNum)?;
                    food.push(n);
                }
            }

            Ok(Self { elves_food })
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
