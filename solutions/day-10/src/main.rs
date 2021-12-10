const INPUT: &str = include_str!("../../../inputs/day-10.txt");

fn main() -> anyhow::Result<()> {
    let mut invalid_chars = Vec::new();
    for line in INPUT.lines() {
        let mut stack = Vec::new();
        if let Err(c) = parse(&mut stack, line) {
            invalid_chars.push(c);
        }
    }

    let answer = invalid_chars
        .into_iter()
        .map(|c| {
            if c == ')' {
                3
            } else if c == ']' {
                57
            } else if c == '}' {
                1197
            } else if c == '>' {
                25137
            } else {
                0
            }
        })
        .sum::<usize>();
    println!("first answer is {}", answer);

    let mut scores = Vec::new();
    for line in INPUT.lines() {
        let mut stack = Vec::new();
        if parse(&mut stack, line).is_ok() {
            stack.reverse();
            let score = stack
                .into_iter()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    c => unreachable!("{}", c),
                })
                .fold(0usize, |acc, s| acc * 5 + s);
            scores.push(score);
        }
    }

    scores.sort_unstable();
    let answer = scores[scores.len() / 2];
    println!("second answer is {}", answer);

    Ok(())
}

fn parse(stack: &mut Vec<char>, line: &str) -> Result<(), char> {
    let mut iter = line.chars();
    match iter.next().map(|c| (c, iter.as_str())) {
        Some((c, tail)) => {
            match c {
                '(' => stack.push(c),
                ')' => {
                    if stack.last().map_or(false, |t| *t == '(') {
                        stack.pop();
                    } else {
                        return Err(c);
                    }
                }
                '[' => stack.push(c),
                ']' => {
                    if stack.last().map_or(false, |t| *t == '[') {
                        stack.pop();
                    } else {
                        return Err(c);
                    }
                }
                '{' => stack.push(c),
                '}' => {
                    if stack.last().map_or(false, |t| *t == '{') {
                        stack.pop();
                    } else {
                        return Err(c);
                    }
                }
                '<' => stack.push(c),
                '>' => {
                    if stack.last().map_or(false, |t| *t == '<') {
                        stack.pop();
                    } else {
                        return Err(c);
                    }
                }
                c => panic!("whats that {}?", c),
            }

            parse(stack, tail)
        }
        None => Ok(()),
    }
}
