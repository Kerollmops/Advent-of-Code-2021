use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../../inputs/day-8.txt");

fn main() -> anyhow::Result<()> {
    let answers = INPUT
        .lines()
        .filter_map(|l| l.split_once('|'))
        .flat_map(|(_, s)| s.split_whitespace())
        .collect::<Vec<&str>>();

    let answer = answers
        .into_iter()
        .filter(|l| l.len() == 2 || l.len() == 4 || l.len() == 3 || l.len() == 7)
        .count();

    println!("first answer is {}", answer);

    let input = INPUT
        .lines()
        .filter_map(|l| l.split_once('|'))
        .map(|(l, s)| {
            (
                l.split_whitespace()
                    .map(|s| s.chars().collect())
                    .collect::<Vec<Vec<_>>>(),
                s.split_whitespace()
                    .map(|s| s.chars().collect())
                    .collect::<Vec<Vec<_>>>(),
            )
        });

    let mut answer = 0;
    for (mut left, right) in input {
        // Move the more restrictive numbers first to speed up the search.
        left.sort_unstable_by_key(|l| {
            if l.len() == 2 {
                0
            } else if l.len() == 3 {
                1
            } else if l.len() == 4 {
                2
            } else if l.len() == 7 {
                3
            } else {
                4
            }
        });

        let map = SevenSegments::new();
        match backtrack(&left, map) {
            Output::NoSolution => panic!("Impossible to find a solution"),
            Output::SolutionFound(map) => {
                let mut num = 0;
                for n in right {
                    let n = chars_to_number(&map, &n).unwrap();
                    num = num * 10 + n;
                }

                answer += num;
            }
        }
    }

    println!("second answer is {}", answer);

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Segment {
    Top,
    TopLeft,
    TopRight,
    Middle,
    BottomLeft,
    BottomRight,
    Bottom,
}

type SevenSegments = HashMap<Segment, char>;

#[derive(Clone)]
enum Output {
    NoSolution,
    SolutionFound(SevenSegments),
}

fn backtrack(combinations: &[Vec<char>], map: SevenSegments) -> Output {
    match combinations.split_first() {
        Some((first_chars, tail)) => {
            let possible_numbers = Number::from_chars(&first_chars);
            assert!(!possible_numbers.is_empty());

            for number in possible_numbers {
                let possible_new_maps = number.possible_new_maps(&map);
                if possible_new_maps.is_empty() {
                    continue;
                }

                for new_map in possible_new_maps {
                    if let Output::SolutionFound(map) = backtrack(tail, new_map) {
                        return Output::SolutionFound(map);
                    }
                }
            }

            Output::NoSolution
        }
        None => Output::SolutionFound(map),
    }
}

fn chars_to_number(map: &SevenSegments, chars: &[char]) -> Option<usize> {
    use Segment::*;

    let mut segments: Vec<_> = chars
        .iter()
        .copied()
        .map(|x| map.iter().find(|(_, c)| x == **c).unwrap().0)
        .collect();
    segments.sort_unstable();

    match &segments[..] {
        [Top, TopLeft, TopRight, BottomLeft, BottomRight, Bottom] => Some(0),
        [TopRight, BottomRight] => Some(1),
        [Top, TopRight, Middle, BottomLeft, Bottom] => Some(2),
        [Top, TopRight, Middle, BottomRight, Bottom] => Some(3),
        [TopLeft, TopRight, Middle, BottomRight] => Some(4),
        [Top, TopLeft, Middle, BottomRight, Bottom] => Some(5),
        [Top, TopLeft, Middle, BottomLeft, BottomRight, Bottom] => Some(6),
        [Top, TopRight, BottomRight] => Some(7),
        [Top, TopLeft, TopRight, Middle, BottomLeft, BottomRight, Bottom] => Some(8),
        [Top, TopLeft, TopRight, Middle, BottomRight, Bottom] => Some(9),
        _ => None,
    }
}

#[derive(Debug, Copy, Clone)]
enum Number {
    Zero([char; 6]),
    One([char; 2]),
    Two([char; 5]),
    Three([char; 5]),
    Four([char; 4]),
    Five([char; 5]),
    Six([char; 6]),
    Seven([char; 3]),
    Eight([char; 7]),
    Nine([char; 6]),
}

impl Number {
    fn from_chars(chars: &[char]) -> Vec<Number> {
        match chars.len() {
            2 => vec![Number::One(chars.try_into().unwrap())],
            3 => vec![Number::Seven(chars.try_into().unwrap())],
            4 => vec![Number::Four(chars.try_into().unwrap())],
            5 => vec![
                Number::Two(chars.try_into().unwrap()),
                Number::Three(chars.try_into().unwrap()),
                Number::Five(chars.try_into().unwrap()),
            ],
            6 => vec![
                Number::Zero(chars.try_into().unwrap()),
                Number::Six(chars.try_into().unwrap()),
                Number::Nine(chars.try_into().unwrap()),
            ],
            7 => vec![Number::Eight(chars.try_into().unwrap())],
            _ => vec![],
        }
    }

    fn possible_new_maps(&self, map: &SevenSegments) -> Vec<SevenSegments> {
        fn possible_seven_segments(
            chars: &[char],
            segments: &[Segment],
            map: &SevenSegments,
        ) -> Vec<HashMap<Segment, char>> {
            chars
                .into_iter()
                .copied()
                .permutations(chars.len())
                .filter_map(|chars| {
                    let mut map = map.clone();
                    for (s, c) in segments.into_iter().copied().zip(chars) {
                        if !map.insert(s, c).map_or(true, |old| old == c) {
                            return None;
                        }
                    }
                    Some(map)
                })
                .collect()
        }

        use Segment::*;

        match self {
            Number::Zero(chars) => possible_seven_segments(
                &chars[..],
                &[Top, TopLeft, TopRight, BottomLeft, BottomRight, Bottom],
                map,
            ),
            Number::One(chars) => {
                possible_seven_segments(&chars[..], &[TopRight, BottomRight], map)
            }
            Number::Two(chars) => possible_seven_segments(
                &chars[..],
                &[Top, TopRight, Middle, BottomLeft, Bottom],
                map,
            ),
            Number::Three(chars) => possible_seven_segments(
                &chars[..],
                &[Top, TopRight, Middle, BottomRight, Bottom],
                map,
            ),
            Number::Four(chars) => {
                possible_seven_segments(&chars[..], &[TopLeft, TopRight, Middle, BottomRight], map)
            }
            Number::Five(chars) => possible_seven_segments(
                &chars[..],
                &[Top, TopLeft, Middle, BottomRight, Bottom],
                map,
            ),
            Number::Six(chars) => possible_seven_segments(
                &chars[..],
                &[Top, TopLeft, Middle, BottomLeft, BottomRight, Bottom],
                map,
            ),
            Number::Seven(chars) => {
                possible_seven_segments(&chars[..], &[Top, TopRight, BottomRight], map)
            }
            Number::Eight(chars) => possible_seven_segments(
                &chars[..],
                &[
                    Top,
                    TopLeft,
                    TopRight,
                    Middle,
                    BottomLeft,
                    BottomRight,
                    Bottom,
                ],
                map,
            ),
            Number::Nine(chars) => possible_seven_segments(
                &chars[..],
                &[Top, TopLeft, TopRight, Middle, BottomRight, Bottom],
                map,
            ),
        }
    }
}
