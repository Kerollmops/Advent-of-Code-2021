use ndarray::Array2;
use std::cmp::Reverse;

const INPUT: &str = include_str!("../../../inputs/day-9.txt");

fn main() -> anyhow::Result<()> {
    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();

    let mut map = Array2::default((height, width));
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let n = (c as u8 - b'0') as u32;
            map[(y, x)] = n;
        }
    }

    let mut lowests = Vec::new();
    for ((y, x), v) in map.indexed_iter() {
        let top = map.get((y.wrapping_sub(1), x));
        let left = map.get((y, x.wrapping_sub(1)));
        let right = map.get((y, x + 1));
        let bottom = map.get((y + 1, x));

        if top.map_or(true, |x| x > v)
            && left.map_or(true, |x| x > v)
            && right.map_or(true, |x| x > v)
            && bottom.map_or(true, |x| x > v)
        {
            lowests.push(v);
        }
    }

    let answer = lowests.into_iter().map(|v| *v as usize + 1).sum::<usize>();
    println!("first answer is {}", answer);

    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();

    let mut map = Array2::default((height, width));
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let n = (c as u8 - b'0') as u32;
            map[(y, x)] = Some(n);
        }
    }

    let mut groups = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let size = recursive_spread(&mut map, y, x);
            if size != 0 {
                groups.push(size)
            }
        }
    }

    groups.sort_unstable_by_key(|size| Reverse(*size));

    let answer = groups.into_iter().take(3).product::<usize>();
    println!("second answer is {}", answer);

    Ok(())
}

fn recursive_spread(map: &mut Array2<Option<u32>>, y: usize, x: usize) -> usize {
    if map
        .get((y, x))
        .map_or(false, |o| o.map_or(false, |v| v != 9))
    {
        map[(y, x)] = None;
        let top = recursive_spread(map, y.wrapping_sub(1), x);
        let left = recursive_spread(map, y, x.wrapping_sub(1));
        let right = recursive_spread(map, y, x + 1);
        let bottom = recursive_spread(map, y + 1, x);
        top + left + right + bottom + 1
    } else {
        0
    }
}
