use ndarray::Array2;
use std::str::FromStr;

const INPUT: &str = include_str!("../../../inputs/day-13.txt");

fn main() -> anyhow::Result<()> {
    let mut points = Vec::new();
    let mut parse_instructions = false;
    let mut folds = Vec::new();

    for line in INPUT.lines() {
        if !line.is_empty() {
            if parse_instructions {
                let fold = line.strip_prefix("fold along ").unwrap();
                let fold = Fold::from_str(fold).unwrap();
                folds.push(fold);
            } else {
                let (x, y) = line.split_once(',').unwrap();
                let x: u32 = x.parse().unwrap();
                let y: u32 = y.parse().unwrap();
                points.push((x, y));
            }
        } else {
            parse_instructions = true;
        }
    }

    for fold in folds.iter().copied().take(1) {
        match fold {
            Fold::X(n) => {
                points.sort_unstable_by_key(|(x, _)| *x);
                let offset = points.iter().position(|(x, _)| *x >= n).unwrap();
                let (_, right) = points.split_at_mut(offset);
                right.iter_mut().for_each(|(x, _)| *x -= (*x - n) * 2);
            }
            Fold::Y(n) => {
                points.sort_unstable_by_key(|(_, y)| *y);
                let offset = points.iter().position(|(_, y)| *y >= n).unwrap();
                let (_, bottom) = points.split_at_mut(offset);
                bottom.iter_mut().for_each(|(_, y)| *y -= (*y - n) * 2);
            }
        }
    }

    points.sort_unstable();
    points.dedup();

    let answer = points.len();
    println!("first answer is {}", answer);

    for fold in folds {
        match fold {
            Fold::X(n) => {
                points.sort_unstable_by_key(|(x, _)| *x);
                if let Some(offset) = points.iter().position(|(x, _)| *x >= n) {
                    let (_, right) = points.split_at_mut(offset);
                    right.iter_mut().for_each(|(x, _)| *x -= (*x - n) * 2);
                }
            }
            Fold::Y(n) => {
                points.sort_unstable_by_key(|(_, y)| *y);
                if let Some(offset) = points.iter().position(|(_, y)| *y >= n) {
                    let (_, bottom) = points.split_at_mut(offset);
                    bottom.iter_mut().for_each(|(_, y)| *y -= (*y - n) * 2);
                }
            }
        }
    }

    points.sort_unstable();
    points.dedup();

    let (max_x, max_y) = points.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max_x.max(*x), max_y.max(*y))
    });

    let mut map = Array2::from_elem((max_y as usize + 1, max_x as usize + 1), '.');
    points
        .iter()
        .for_each(|(x, y)| map[(*y as usize, *x as usize)] = '#');

    println!("second answer is");
    println!("{}", map);

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    X(u32),
    Y(u32),
}

impl FromStr for Fold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Fold> {
        let (axis, n) = s.split_once('=').unwrap();
        let n = n.parse()?;
        match axis {
            "x" => Ok(Fold::X(n)),
            "y" => Ok(Fold::Y(n)),
            _ => Err(anyhow::anyhow!("What's axis {}?", axis)),
        }
    }
}
