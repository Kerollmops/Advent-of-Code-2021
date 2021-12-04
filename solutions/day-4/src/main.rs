use self::Number::{Marked, UnMarked};
use std::cmp::Ordering;
use std::mem;

const INPUT: &str = include_str!("../../../inputs/day-4.txt");
const BOARD_SIZE: usize = 5;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Number {
    UnMarked(i32),
    Marked(i32),
}

impl Number {
    fn unmarked(&self) -> Option<i32> {
        match self {
            UnMarked(i) => Some(*i),
            Marked(_) => None,
        }
    }
}

struct Board {
    inner: [[Number; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn from_numbers(numbers: impl IntoIterator<Item = Number>) -> Board {
        let mut inner = [[UnMarked(0); BOARD_SIZE]; BOARD_SIZE];
        for (i, n) in numbers.into_iter().enumerate() {
            let (y, x) = (i / BOARD_SIZE, i % 5);
            inner[y][x] = n;
        }
        Board { inner }
    }

    fn mark_number(&mut self, number: i32) {
        for line in &mut self.inner {
            for x in line {
                if *x == UnMarked(number) {
                    *x = Marked(number);
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        for line in self.inner {
            if line.iter().all(|x| matches!(x, Marked(_))) {
                return true;
            }
        }

        for i in 0..BOARD_SIZE {
            if (0..BOARD_SIZE)
                .map(|j| self.inner[j][i])
                .all(|x| matches!(x, Marked(_)))
            {
                return true;
            }
        }

        return false;
    }

    fn unmarked_numbers(&self) -> impl Iterator<Item = i32> + '_ {
        self.inner.iter().flatten().filter_map(|n| n.unmarked())
    }
}

fn main() -> anyhow::Result<()> {
    let lines: Vec<_> = INPUT.lines().collect();
    let mut lines = lines.into_iter();

    let random_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse())
        .collect::<Result<Vec<i32>, _>>()?;
    assert!(lines.next().unwrap().is_empty());

    let mut boards = Vec::new();
    let mut retrieved_numbers = Vec::<Number>::new();
    for line in lines {
        if line.is_empty() {
            let board = Board::from_numbers(mem::take(&mut retrieved_numbers));
            boards.push(board);
        } else {
            let line_numbers = line
                .split_whitespace()
                .map(|n| n.parse().map(UnMarked))
                .collect::<Result<Vec<_>, _>>()?;
            retrieved_numbers.extend(line_numbers);
        }
    }

    let board = Board::from_numbers(mem::take(&mut retrieved_numbers));
    boards.push(board);

    'big: for n in &random_numbers {
        for board in &mut boards {
            board.mark_number(*n);
            if board.is_complete() {
                let answer = board.unmarked_numbers().sum::<i32>() * n;
                println!("first answer is {}", answer);
                break 'big;
            }
        }
    }

    // Part Two

    let mut boards: Vec<_> = boards.into_iter().map(Some).collect();
    let mut remaining_boards = boards.len();

    'big2: for n in random_numbers {
        for optional_board in &mut boards {
            if let Some(board) = optional_board {
                board.mark_number(n);
                if board.is_complete() {
                    if remaining_boards == 1 {
                        // is last board
                        let answer = board.unmarked_numbers().sum::<i32>() * n;
                        println!("second answer is {}", answer);
                        break 'big2;
                    }
                    *optional_board = None;
                    remaining_boards -= 1;
                }
            }
        }
    }

    Ok(())
}
