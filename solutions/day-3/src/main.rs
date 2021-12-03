use bitvec::prelude::*;

const INPUT: &str = include_str!("../../../inputs/day-3.txt");

fn main() -> anyhow::Result<()> {
    let bits_lines: Vec<_> = INPUT.lines().collect();
    let number_lines = bits_lines.len();
    let length = bits_lines[0].len();

    let mut ones_counts = vec![0; length];
    for bits in &bits_lines {
        for (bit, count) in bits.chars().zip(&mut ones_counts) {
            *count += (bit == '1') as usize;
        }
    }

    let mut gamma_bits: BitVec = BitVec::new();
    let mut alpha_bits: BitVec = BitVec::new();
    for (i, count) in ones_counts.into_iter().enumerate() {
        if dbg!(count > number_lines / 2) {
            gamma_bits.push(true);
            alpha_bits.push(false);
        } else {
            gamma_bits.push(false);
            alpha_bits.push(true);
        }
    }

    gamma_bits.reverse();
    alpha_bits.reverse();

    let gamma = gamma_bits.into_vec()[0];
    let alpha = alpha_bits.into_vec()[0];
    let answer = gamma * alpha;

    println!("first answer is {}", answer);

    Ok(())
}
