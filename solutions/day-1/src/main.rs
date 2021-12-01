use std::str::FromStr;

const INPUT: &str = include_str!("../../../inputs/day-1.txt");

fn main() -> anyhow::Result<()> {
    let depths = INPUT
        .lines()
        .map(u32::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let count = depths
        .windows(2)
        .fold(0, |acc, depths| acc + (depths[0] < depths[1]) as usize);

    println!("first answer is {}", count);

    Ok(())
}
