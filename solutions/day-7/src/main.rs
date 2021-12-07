use std::cmp::Ordering;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../../../inputs/day-7.txt");

fn main() -> anyhow::Result<()> {
    let input = INPUT
        .split(',')
        .map(|n| n.trim().parse())
        .collect::<Result<Vec<usize>, _>>()?;

    let mut pos_counts = Vec::new();
    for pos in input {
        if pos_counts.len() < pos + 1 {
            pos_counts.resize(pos + 1, 0);
        }
        pos_counts[pos] += 1;
    }

    let most_pop: BinaryHeap<_> = pos_counts
        .iter()
        .copied()
        .enumerate()
        .map(|(pos, count)| PosCount { pos, count })
        .collect();

    let answer = most_pop
        .clone()
        .into_iter()
        .map(|pc| cost_to_move(&pos_counts, pc.pos))
        .min()
        .unwrap();

    println!("first answer is {}", answer);

    let answer = most_pop
        .into_iter()
        .map(|pc| cost_to_move2(&pos_counts, pc.pos))
        .min()
        .unwrap();

    println!("second answer is {}", answer);

    Ok(())
}

fn cost_to_move(pop_counts: &[usize], pos: usize) -> usize {
    let mut cost = 0;
    for (i, count) in pop_counts.iter().enumerate() {
        cost += if pos > i { pos - i } else { i - pos } * count;
    }
    cost
}

fn cost_to_move2(pop_counts: &[usize], pos: usize) -> usize {
    let mut cost = 0;
    for (i, count) in pop_counts.iter().enumerate() {
        let dist = if pos > i { pos - i } else { i - pos };
        cost += (dist * (dist + 1)) / 2 * count;
    }
    cost
}

#[derive(Clone)]
struct PosCount {
    pos: usize,
    count: usize,
}

impl PartialOrd for PosCount {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(&rhs))
    }
}

impl Ord for PosCount {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.count.cmp(&rhs.count)
    }
}

impl PartialEq for PosCount {
    fn eq(&self, rhs: &Self) -> bool {
        self.count == rhs.count
    }
}

impl Eq for PosCount {}
