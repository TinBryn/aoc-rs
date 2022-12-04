fn main() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("./input/y22/d04/input.txt")?;
    let input: Input = input.parse()?;
    problem1(&input);
    problem2(&input);

    Ok(())
}

fn fully_overlap(a: &Assignment) -> bool {
    a.first.0 <= a.second.0 && a.first.1 >= a.second.1
        || a.first.1 <= a.second.1 && a.first.0 >= a.second.0
}

fn dont_overlap(a: &Assignment) -> bool {
    a.first.1 < a.second.0 || a.second.1 < a.first.0
}

fn problem1(input: &Input) {
    let total = input.lines.iter().filter(|a| fully_overlap(a)).count();
    println!("total overlaps: {}", total);
}

fn problem2(input: &Input) {
    let total = input.lines.iter().filter(|a| !dont_overlap(a)).count();
    println!("total overlaps: {}", total);
}

struct Assignment {
    first: (u64, u64),
    second: (u64, u64),
}

pub struct Input {
    lines: Vec<Assignment>,
}

impl Input {}

mod parsing {
    use std::{num::ParseIntError, str::FromStr};

    use crate::Assignment;

    #[derive(Debug)]
    pub enum AssignmentParseError {
        Missing,
        IntParse,
    }

    impl From<ParseIntError> for AssignmentParseError {
        fn from(_: ParseIntError) -> Self {
            Self::IntParse
        }
    }

    impl FromStr for super::Input {
        type Err = AssignmentParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let lines = s.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;

            Ok(Self { lines })
        }
    }

    impl FromStr for Assignment {
        type Err = AssignmentParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (first, second) = s.split_once(',').ok_or(AssignmentParseError::Missing)?;
            let first = first.trim();
            let second = second.trim();
            let (a, b) = first.split_once('-').ok_or(AssignmentParseError::Missing)?;
            let (c, d) = second
                .split_once('-')
                .ok_or(AssignmentParseError::Missing)?;
            let a = a.parse()?;
            let b = b.parse()?;
            let c = c.parse()?;
            let d = d.parse()?;

            Ok(Self {
                first: (a, b),
                second: (c, d),
            })
        }
    }
}

mod error {
    use crate::parsing::AssignmentParseError;

    #[derive(Debug)]
    pub enum Error {
        IO(std::io::Error),
        Parse(AssignmentParseError),
    }

    impl From<std::io::Error> for Error {
        fn from(err: std::io::Error) -> Self {
            Self::IO(err)
        }
    }

    impl From<AssignmentParseError> for Error {
        fn from(err: AssignmentParseError) -> Self {
            Self::Parse(err)
        }
    }
}
