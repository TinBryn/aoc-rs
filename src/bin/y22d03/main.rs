fn main() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("./input/y22/d03/input.txt")?;
    let input: Input = input.parse()?;
    problem1(&input);
    problem2(&input);

    Ok(())
}

fn problem1(_input: &Input) {
    let total: u64 = _input
        .lines
        .iter()
        .map(|line| {
            let c = Input::find_duplicate(line).unwrap();

            Input::priority(c)
        })
        .sum();
    println!("total priority: {total}");
}

fn problem2(_input: &Input) {
    use itertools::Itertools;
    let total: u64 = _input
        .lines
        .iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let chunk: Vec<_> = chunk.into_iter().collect();
            let c = Input::find_common(chunk[0], chunk[1], chunk[2]).unwrap();
            Input::priority(c)
        })
        .sum();
    println!("total badge {total}");
}

pub struct Input {
    lines: Vec<String>,
}

impl Input {
    fn find_duplicate(line: &str) -> Option<char> {
        let (left, right) = line.split_at(line.len() / 2);

        right.chars().find(|&c| left.contains(c))
    }

    fn priority(item: char) -> u64 {
        match item {
            'a'..='z' => (item as u8 - b'a' + 1) as u64,
            'A'..='Z' => (item as u8 - b'A' + 27) as u64,
            _ => panic!("invalid"),
        }
    }

    fn find_common(astr: &str, bstr: &str, cstr: &str) -> Option<char> {
        astr.chars().find(|&a| bstr.contains(a) && cstr.contains(a))
    }
}

mod parsing {
    use std::str::FromStr;

    #[derive(Debug)]
    pub enum InputParseError {
        MissingPart,
        Invalid,
    }

    impl FromStr for super::Input {
        type Err = InputParseError;
        fn from_str(_s: &str) -> Result<Self, Self::Err> {
            let lines = _s.lines().map(|l| l.to_owned()).collect();

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
