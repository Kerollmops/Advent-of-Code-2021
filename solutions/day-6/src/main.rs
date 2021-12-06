const INPUT: &str = include_str!("../../../inputs/day-6.txt");

fn main() -> anyhow::Result<()> {
    let ages = INPUT
        .split(',')
        .map(|n| n.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    let answer = run_for_days(ages.iter().copied(), 80);
    println!("first answer is {}", answer);

    let answer = run_for_days(ages, 256);
    println!("second answer is {}", answer);

    Ok(())
}

fn run_for_days<I: IntoIterator<Item = usize>>(ages: I, days: usize) -> usize {
    let mut cooldown_count = [0; 9]; // from 1 to 9
    ages.into_iter().for_each(|c| cooldown_count[c] += 1);

    for day in 0..days {
        let newborns = cooldown_count[0];
        cooldown_count.rotate_left(1);
        cooldown_count[6] += newborns;
        cooldown_count[8] = newborns;
    }

    cooldown_count.into_iter().sum::<usize>()
}
