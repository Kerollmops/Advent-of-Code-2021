use anyhow::bail;

const INPUT: &str = include_str!("../../../inputs/day-5.txt");

type Position = [i32; 2];

fn main() -> anyhow::Result<()> {
    let mut moves = INPUT
        .lines()
        .map(parse_move)
        .collect::<Result<Vec<_>, _>>()?;

    moves.retain(|&(s, e)| is_horizontal(s, e) || is_vertical(s, e));

    let mut map = vec![vec![0; 1000]; 1000];
    for (s, e) in moves {
        draw_line(&mut map, s, e)?;
    }

    // display_map(&map);

    let answer = count_on_map(&map, |tile| *tile >= 2);
    println!("first answer is {}", answer);

    // Part Two

    let mut moves = INPUT
        .lines()
        .map(parse_move)
        .collect::<Result<Vec<_>, _>>()?;

    moves.retain(|&(s, e)| is_horizontal(s, e) || is_vertical(s, e) || is_diagonal(s, e));

    let mut map = vec![vec![0; 1000]; 1000];
    for (s, e) in moves {
        draw_line(&mut map, s, e)?;
    }

    // display_map(&map);

    let answer = count_on_map(&map, |tile| *tile >= 2);
    println!("second answer is {}", answer);

    Ok(())
}

fn display_map(map: &Vec<Vec<usize>>) {
    for line in map {
        for tile in line {
            print!("{}", tile);
        }
        println!();
    }
    println!();
}

fn count_on_map<F: Fn(&usize) -> bool>(map: &Vec<Vec<usize>>, f: F) -> usize {
    map.iter()
        .map(|line| line.iter().filter(|t| (f)(*t)).count())
        .sum::<usize>()
}

fn draw_line(map: &mut Vec<Vec<usize>>, s: Position, e: Position) -> anyhow::Result<()> {
    if is_horizontal(s, e) {
        let horizon = s[1] as usize;
        let (s, e) = small_first(s[0], e[0]);

        for i in s..=e {
            map[horizon][i as usize] += 1;
        }

        Ok(())
    } else if is_vertical(s, e) {
        let vertical = s[0] as usize;
        let (s, e) = small_first(s[1], e[1]);

        for i in s..=e {
            map[i as usize][vertical] += 1;
        }

        Ok(())
    } else if is_diagonal(s, e) {
        let (s, e) = clean_diagonal(s, e);
        if is_top_bottom_diagonal(s, e) {
            for i in 0..=diagonal_length(s, e) {
                map[s[1] as usize + i][s[0] as usize + i] += 1;
            }

            Ok(())
        } else {
            for i in 0..=diagonal_length(s, e) {
                map[s[1] as usize + i][s[0] as usize - i] += 1;
            }

            Ok(())
        }
    } else {
        bail!("can't draw non-horizontal or non-vertical lines")
    }
}

fn is_horizontal(s: Position, e: Position) -> bool {
    s[1] == e[1]
}

fn is_vertical(s: Position, e: Position) -> bool {
    s[0] == e[0]
}

fn is_diagonal(s: Position, e: Position) -> bool {
    let vertical_dist = (s[0] - e[0]).abs();
    let horizontal_dist = (s[1] - e[1]).abs();
    vertical_dist == horizontal_dist
}

fn diagonal_length(s: Position, e: Position) -> usize {
    (s[0] - e[0]).abs() as usize
}

/// Makes the diagonal either left-right top-bottom or left-right bottom-top
fn clean_diagonal(s: Position, e: Position) -> (Position, Position) {
    assert!(is_diagonal(s, e));
    if s[1] > e[1] {
        (e, s)
    } else {
        (s, e)
    }
}

fn is_top_bottom_diagonal(s: Position, e: Position) -> bool {
    assert!(is_diagonal(s, e));
    s[0] < e[0]
}

fn parse_move(s: &str) -> anyhow::Result<(Position, Position)> {
    match s.split_once("->") {
        Some((l, r)) => {
            let left = match l.trim().split_once(',') {
                Some((x, y)) => [x.parse()?, y.parse()?],
                None => bail!("invalid left position"),
            };

            let right = match r.trim().split_once(',') {
                Some((x, y)) => [x.parse()?, y.parse()?],
                None => bail!("invalid right position"),
            };

            Ok((left, right))
        }
        None => bail!("no `->` found"),
    }
}

fn small_first(a: i32, b: i32) -> (i32, i32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}
