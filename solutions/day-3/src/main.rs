use bitvec::prelude::*;
use std::cmp::Ordering;

const INPUT: &str = include_str!("../../../inputs/day-3.txt");

fn main() {
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
    for count in ones_counts.into_iter() {
        if count > number_lines / 2 {
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

    let mut oxygen_generator_rating_lines = bits_lines.clone();
    let oxygen_generator_rating = filter_lines(&mut oxygen_generator_rating_lines, 0, Common::Most);
    let oxygen_generator_rating = isize::from_str_radix(oxygen_generator_rating, 2).unwrap();

    let mut co2_scrubber_rating_lines = bits_lines.clone();
    let co2_scrubber_rating = filter_lines(&mut co2_scrubber_rating_lines, 0, Common::Least);
    let co2_scrubber_rating = isize::from_str_radix(co2_scrubber_rating, 2).unwrap();

    let answer = oxygen_generator_rating * co2_scrubber_rating;
    println!("second answer is {}", answer);
}

enum Common {
    Least,
    Most,
}

fn filter_lines<'a>(lines: &mut Vec<&'a str>, position: usize, criteria: Common) -> &'a str {
    if lines.len() == 1 {
        lines[0]
    } else {
        let mut ones_count = 0;
        let mut zeros_count = 0;
        for line in lines.iter() {
            if line.chars().nth(position) == Some('1') {
                ones_count += 1;
            } else {
                zeros_count += 1;
            }
        }

        let keep_bit = match ones_count.cmp(&zeros_count) {
            Ordering::Less => match criteria {
                Common::Most => '0',
                Common::Least => '1',
            },
            Ordering::Equal => match criteria {
                Common::Most => '1',
                Common::Least => '0',
            },
            Ordering::Greater => match criteria {
                Common::Most => '1',
                Common::Least => '0',
            },
        };

        lines.retain(|l| l.chars().nth(position) == Some(keep_bit));

        filter_lines(lines, position + 1, criteria)
    }
}
