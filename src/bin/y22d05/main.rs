fn main() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("./input/y22/d05/input.txt")?;
    let input: Input = input.parse()?;
    problem1(&input);
    problem2(&input);

    Ok(())
}

fn problem1(input: &Input) {}

fn problem2(input: &Input) {}

pub struct Input {
    lines: Vec<String>,
}

impl Input {}

mod parsing {
    use std::{num::ParseIntError, str::FromStr};

    #[derive(Debug)]
    pub enum InputParseError {
        Missing,
        IntParse,
    }

    impl From<ParseIntError> for InputParseError {
        fn from(_: ParseIntError) -> Self {
            Self::IntParse
        }
    }

    impl FromStr for super::Input {
        type Err = InputParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let lines = s.lines().map(|l| l.to_owned()).collect::<_>();

            Ok(Self { lines })
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

#[cfg(test)]
mod test {
    use crate::{error::Error, Input, problem1, problem2};

    #[test]
    fn test() -> Result<(), Error> {
        let input = std::fs::read_to_string("./input/y22/d05/test.txt")?;
        let input: Input = input.parse()?;
        problem1(&input);
        problem2(&input);

        Ok(())
    }
}
