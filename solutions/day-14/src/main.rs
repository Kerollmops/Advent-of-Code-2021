use std::collections::HashMap;
use std::mem;

const INPUT: &str = include_str!("../../../inputs/day-14.txt");

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();
    let input: Vec<_> = lines.next().unwrap().chars().collect();
    assert!(lines.next().unwrap().is_empty());
    let input_patterns: HashMap<_, _> = lines
        .filter_map(|l| {
            l.split_once(" -> ").map(|(k, v)| {
                let a = k.chars().nth(0).unwrap();
                let b = k.chars().nth(1).unwrap();
                ([a, b], v.chars().next().unwrap())
            })
        })
        .collect();

    let mut template: Vec<_> = input.clone();
    for _ in 0..10 {
        let mut i = 0;
        while let Some((a, b)) = template.get(i).zip(template.get(i + 1)) {
            if let Some(c) = input_patterns.get(&[*a, *b][..]) {
                template.insert(i + 1, *c);
            }
            i += 2;
        }
    }

    let counts = template.into_iter().fold(HashMap::new(), |mut counts, c| {
        *counts.entry(c).or_insert(0) += 1;
        counts
    });

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    let answer = max - min;
    println!("first answer is {}", answer);

    // Init the number of patterns.
    let mut patterns_count = HashMap::new();
    let last_char = input.last().unwrap();
    for win in input.windows(2) {
        let pattern = [win[0], win[1]];
        *patterns_count.entry(pattern).or_insert(0) += 1usize;
    }

    for _ in 0..40 {
        for (pattern @ [a, b], count) in mem::take(&mut patterns_count) {
            if let Some(&n) = input_patterns.get(&pattern) {
                *patterns_count.entry([a, n]).or_insert(0) += count;
                *patterns_count.entry([n, b]).or_insert(0) += count;
            }
        }
    }

    let mut letter_counts = HashMap::new();
    for ([a, _], count) in patterns_count {
        *letter_counts.entry(a).or_insert(0) += count;
    }

    // Don't forget to add the last letter to the map.
    *letter_counts.entry(*last_char).or_insert(0) += 1;

    let max = letter_counts.values().max().unwrap();
    let min = letter_counts.values().min().unwrap();

    let answer = max - min;
    println!("second answer is {}", answer);

    Ok(())
}
