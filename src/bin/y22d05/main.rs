fn main() -> Result<(), error::Error> {
    let input = std::fs::read_to_string("./input/y22/d05/input.txt")?;
    let input: Input = input.parse()?;
    problem1(&input);
    problem2(&input);

    Ok(())
}

fn problem1(input: &Input) {
    let mut stacks = input.stacks.clone();

    for &Move { amount, from, to } in &input.moves {
        for _ in 0..amount {
            let c = stacks[from - 1].stack.pop().unwrap();
            stacks[to - 1].stack.push(c);
        }
    }

    for s in stacks {
        print!("{}", s.stack.last().unwrap())
    }
    println!();
}

fn problem2(input: &Input) {
    let mut stacks = input.stacks.clone();

    for &Move { amount, from, to } in &input.moves {
        for _ in 0..amount {
            let c = stacks[from - 1].stack.pop().unwrap();
            stacks[to - 1].stack.push(c);
        }
        let len = stacks[to - 1].stack.len();
        stacks[to - 1].stack[(len - amount)..].reverse()
    }

    for s in stacks {
        print!("{}", s.stack.last().unwrap())
    }
    println!();
}

#[derive(Debug)]
pub struct Input {
    stacks: Vec<Stack>,

    moves: Vec<Move>,
}

#[derive(Debug, Clone)]
pub struct Stack {
    stack: Vec<char>,
}

#[derive(Debug)]
pub struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl Input {}

mod parsing {
    use std::{num::ParseIntError, str::FromStr};

    use crate::{Move, Stack};

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
            let mut stacks = vec![];
            let mut moves = vec![];
            enum State {
                Stacks,
                Moves,
            }
            let mut state = State::Stacks;
            'lines: for line in s.lines() {
                match state {
                    State::Stacks => {
                        if line.len() / 4 > stacks.len() {
                            for _ in 0..(line.len() + 1) / 4 {
                                stacks.push(Stack { stack: vec![] })
                            }
                        }
                        for i in (0..line.len()).step_by(4) {
                            let c = line.as_bytes()[i + 1];
                            // c could be 'A' or ' ' or possibly '1'
                            if c == b'1' {
                                state = State::Moves;
                                continue 'lines;
                            }
                            if !c.is_ascii_whitespace() {
                                stacks[i / 4].stack.push(c as char);
                            }
                        }
                    }
                    State::Moves => {
                        if line.is_empty() {
                            continue;
                        }
                        if let Some(rest) = line.strip_prefix("move ") {
                            if let Some((amount, rest)) = rest.split_once("from") {
                                if let Some((from, to)) = rest.split_once("to") {
                                    let amount = amount.trim().parse()?;
                                    let from = from.trim().parse()?;
                                    let to = to.trim().parse()?;
                                    moves.push(Move { amount, from, to });
                                }
                            }
                        }
                    }
                }
            }

            for stack in &mut stacks {
                stack.stack.reverse();
            }

            Ok(Self { stacks, moves })
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
