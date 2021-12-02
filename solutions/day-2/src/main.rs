use std::error::Error;
use std::fmt;
use std::str::FromStr;

const INPUT: &str = include_str!("../../../inputs/day-2.txt");

enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug, Clone, Copy)]
struct InvalidInput;

impl fmt::Display for InvalidInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("invalid input")
    }
}

impl Error for InvalidInput {}

impl FromStr for Movement {
    type Err = InvalidInput;

    fn from_str(s: &str) -> Result<Movement, InvalidInput> {
        match s.split_once(' ') {
            Some((m, c)) => {
                let c = c.trim().parse().map_err(|_| InvalidInput)?;
                match m {
                    "forward" => Ok(Movement::Forward(c)),
                    "down" => Ok(Movement::Down(c)),
                    "up" => Ok(Movement::Up(c)),
                    _ => Err(InvalidInput),
                }
            }
            _ => Err(InvalidInput),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let movements = INPUT
        .lines()
        .map(Movement::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let mut horizontal = 0;
    let mut vertical = 0;

    for movement in &movements {
        match movement {
            Movement::Forward(n) => horizontal += n,
            Movement::Down(n) => vertical += n,
            Movement::Up(n) => vertical -= n,
        }
    }

    let answer = horizontal * vertical;
    println!("first answer is {}", answer);

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for movement in &movements {
        match movement {
            Movement::Forward(n) => {
                horizontal += n;
                depth += aim * n;
            }
            Movement::Down(n) => aim += n,
            Movement::Up(n) => aim -= n,
        }
    }

    let answer = horizontal * depth;
    println!("second answer is {}", answer);

    Ok(())
}
