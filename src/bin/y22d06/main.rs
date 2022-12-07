use std::collections::HashSet;

fn main() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("./input/y22/d06/input.txt")?;
    let input: Input = input.parse()?;
    problem1(&input);
    problem2(&input);

    Ok(())
}

fn problem1(input: &Input) {
    for i in 0..input.0.len() {
        if is_marker(&input.0[i..], 4) {
            println!("{}", i + 4);
            return;
        }
    }
}

fn problem2(input: &Input) {
    for i in 0..input.0.len() {
        if is_marker(&input.0[i..], 14) {
            println!("{}", i + 14);
            return;
        }
    }
}

fn is_marker(s: &str, len: usize) -> bool {
    let set: HashSet<u8> = s.bytes().take(len).collect();

    set.len() == len
}

#[derive(Debug)]
pub struct Input(String);




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
            Ok(Self(s.to_string()))
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
    use crate::{error::Error, problem1, problem2, Input};

    #[test]
    fn test() -> Result<(), Error> {
        let input = std::fs::read_to_string("./input/y22/d05/test.txt")?;
        let input: Input = input.parse()?;
        problem1(&input);
        problem2(&input);

        Ok(())
    }
}
