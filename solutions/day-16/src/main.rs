use bitvec::prelude::*;

const INPUT: &str = include_str!("../../../inputs/day-16.txt");

fn main() -> anyhow::Result<()> {
    let _first_binary: BitVec<usize, Msb0> = [
        true, true, false, true, false, false, true, false, true, true, true, true, true, true,
        true, false, false, false, true, false, true, false, false, false,
    ]
    .into_iter()
    .collect();

    let _second_binary: BitVec<usize, Msb0> = [
        false, false, true, true, true, false, false, false, false, false, false, false, false,
        false, false, false, false, true, true, false, true, true, true, true, false, true, false,
        false, false, true, false, true, false, false, true, false, true, false, false, true,
        false, false, false, true, false, false, true, false, false, false, false, false, false,
        false, false, false,
    ]
    .into_iter()
    .collect();

    let _third_binary: BitVec<usize, Msb0> = [
        true, true, true, false, true, true, true, false, false, false, false, false, false, false,
        false, false, true, true, false, true, false, true, false, false, false, false, false,
        false, true, true, false, false, true, false, false, false, false, false, true, false,
        false, false, true, true, false, false, false, false, false, true, true, false, false,
        false, false, false,
    ]
    .into_iter()
    .collect();

    let binary = hex_to_binary(INPUT.trim());
    let mut version_sum = 0;
    parse_packet(&binary, &mut |v| version_sum += v);

    let answer = version_sum;
    println!("first answer is {}", answer);

    let (answer, _) = parse_packet(&binary, &mut drop);
    println!("second answer is {}", answer);

    Ok(())
}

fn parse_packet<'a, FV: FnMut(u32)>(
    bits: &'a BitSlice<usize, Msb0>,
    fv: &mut FV,
) -> (usize, &'a BitSlice<usize, Msb0>) {
    let (v, bits) = bits.split_at(3);
    let (id, bits) = bits.split_at(3);
    let version = bits_to_number([false, v[0], v[1], v[2]]);
    let type_id = bits_to_number([false, id[0], id[1], id[2]]);

    (fv)(version);

    if type_id == 4 {
        let mut numbits = String::new();
        let mut iter = bits.chunks_exact(5);
        let mut count = 0;
        while let Some(bits) = iter.next() {
            count += bits.len();
            let (first, bits) = bits.split_first().unwrap();
            numbits.extend(bits.into_iter().map(|b| if *b { '1' } else { '0' }));
            if *first == false {
                break;
            }
        }

        let num = usize::from_str_radix(&numbits, 2).unwrap();

        (num, &bits[count..])
    } else {
        let (length_type_id, bits) = bits.split_first().unwrap();
        let (values, rem) = if *length_type_id {
            let (c, mut bits) = bits.split_at(11);
            let s: String = c.into_iter().map(|b| if *b { '1' } else { '0' }).collect();
            let count = usize::from_str_radix(&s, 2).unwrap();
            let mut values = Vec::new();

            for i in 0..count {
                let (val, rem) = parse_packet(bits, fv);
                values.push(val);
                bits = rem;
            }

            (values, bits)
        } else {
            let (l, bits) = bits.split_at(15);
            let s: String = l.into_iter().map(|b| if *b { '1' } else { '0' }).collect();
            let length = usize::from_str_radix(&s, 2).unwrap();
            let (mut sub_bits, bits) = &bits.split_at(length);
            let mut values = Vec::new();

            while sub_bits.len() != 0 {
                let (val, rem) = parse_packet(sub_bits, fv);
                values.push(val);
                sub_bits = rem;
            }

            (values, *bits)
        };

        let value = match Operator::from_type_id(type_id).unwrap() {
            Operator::Sum => values.into_iter().sum::<usize>(),
            Operator::Product => values.into_iter().product::<usize>(),
            Operator::Minimum => values.into_iter().min().unwrap(),
            Operator::Maximum => values.into_iter().max().unwrap(),
            Operator::GreaterThan => (values[0] > values[1]) as usize,
            Operator::LessThan => (values[0] < values[1]) as usize,
            Operator::EqualTo => (values[0] == values[1]) as usize,
        };

        (value, rem)
    }
}

enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operator {
    fn from_type_id(id: u32) -> Option<Operator> {
        match id {
            0 => Some(Operator::Sum),
            1 => Some(Operator::Product),
            2 => Some(Operator::Minimum),
            3 => Some(Operator::Maximum),
            5 => Some(Operator::GreaterThan),
            6 => Some(Operator::LessThan),
            7 => Some(Operator::EqualTo),
            _ => None,
        }
    }
}

fn hex_to_binary(s: &str) -> BitVec<usize, Msb0> {
    s.chars()
        .flat_map(|c| match c {
            '0' => [false, false, false, false],
            '1' => [false, false, false, true],
            '2' => [false, false, true, false],
            '3' => [false, false, true, true],
            '4' => [false, true, false, false],
            '5' => [false, true, false, true],
            '6' => [false, true, true, false],
            '7' => [false, true, true, true],
            '8' => [true, false, false, false],
            '9' => [true, false, false, true],
            'A' => [true, false, true, false],
            'B' => [true, false, true, true],
            'C' => [true, true, false, false],
            'D' => [true, true, false, true],
            'E' => [true, true, true, false],
            'F' => [true, true, true, true],
            _ => panic!("What the heck is a {}!", c),
        })
        .collect()
}

fn bits_to_number(bits: [bool; 4]) -> u32 {
    match bits {
        [false, false, false, false] => 0,
        [false, false, false, true] => 1,
        [false, false, true, false] => 2,
        [false, false, true, true] => 3,
        [false, true, false, false] => 4,
        [false, true, false, true] => 5,
        [false, true, true, false] => 6,
        [false, true, true, true] => 7,
        [true, false, false, false] => 8,
        [true, false, false, true] => 9,
        [true, false, true, false] => 10,
        [true, false, true, true] => 11,
        [true, true, false, false] => 12,
        [true, true, false, true] => 13,
        [true, true, true, false] => 14,
        [true, true, true, true] => 15,
    }
}
