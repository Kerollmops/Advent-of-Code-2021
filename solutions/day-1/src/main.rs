use std::str::FromStr;

const INPUT: &str = include_str!("../../../inputs/day-1.txt");

fn main() -> anyhow::Result<()> {
    let depths = INPUT
        .lines()
        .map(u32::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let count = depths
        .windows(2)
        .filter(|depths| depths[0] < depths[1])
        .count();

    println!("first answer is {}", count);

    let depths_sum_by_3: Vec<u32> = depths
        .windows(3)
        .map(|depth| depth.into_iter().cloned().sum())
        .collect();

    let count = depths_sum_by_3
        .windows(2)
        .filter(|depths| depths[0] < depths[1])
        .count();

    println!("second answer is {}", count);

    Ok(())
}
