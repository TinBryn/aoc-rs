fn main() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("./input/y22/d02/input.txt")?;
    let input: Input = input.parse()?;
    problem1(&input);
    problem2(&input);

    Ok(())
}

fn problem1(_input: &Input) {}

fn problem2(_input: &Input) {}

pub struct Input {
    pub elves_food: Vec<Vec<i64>>,
}

mod parsing {
    use std::str::FromStr;
    #[derive(Debug)]
    pub enum InputParseError {}

    impl FromStr for super::Input {
        type Err = InputParseError;
        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            todo!()
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
