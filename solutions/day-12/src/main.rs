use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = include_str!("../../../inputs/day-12.txt");

fn main() -> anyhow::Result<()> {
    let input = INPUT.lines().filter_map(|l| l.split_once('-'));
    let mut connections = HashMap::new();
    for (a, b) in input {
        let a = Cave::from_str(a)?;
        let b = Cave::from_str(b)?;
        if b != Cave::Start {
            connections
                .entry(a.clone())
                .or_insert_with(Vec::new)
                .push(b.clone());
        }
        if a != Cave::Start && b != Cave::End {
            connections.entry(b).or_insert_with(Vec::new).push(a);
        }
    }

    let answer = recursive_explore(&connections, vec![Cave::Start]);
    println!("first answer is {}", answer);

    let answer = recursive_explore2(&connections, vec![Cave::Start]);
    println!("second answer is {}", answer);

    Ok(())
}

fn recursive_explore(connections: &HashMap<Cave, Vec<Cave>>, path: Vec<Cave>) -> usize {
    if let Some(last) = path.last() {
        if *last == Cave::End {
            1
        } else {
            if let Some(childs) = connections.get(last) {
                let mut count = 0;
                for cave in childs.into_iter() {
                    if !(cave.is_small() && path.contains(cave)) {
                        let mut new_path = path.clone();
                        new_path.push(cave.clone());
                        count += recursive_explore(connections, new_path);
                    }
                }
                count
            } else {
                0
            }
        }
    } else {
        0
    }
}

fn recursive_explore2(connections: &HashMap<Cave, Vec<Cave>>, path: Vec<Cave>) -> usize {
    if let Some(last) = path.last() {
        if *last == Cave::End {
            1
        } else {
            if let Some(childs) = connections.get(last) {
                let mut count = 0;
                for cave in childs.into_iter() {
                    if accept_cave(&path, cave) {
                        let mut new_path = path.clone();
                        new_path.push(cave.clone());
                        count += recursive_explore2(connections, new_path);
                    }
                }
                count
            } else {
                0
            }
        }
    } else {
        0
    }
}

fn accept_cave(path: &[Cave], cave: &Cave) -> bool {
    if cave.is_small() {
        if path.contains(cave) {
            let mut small_occurences = HashMap::new();
            for c in path.iter().filter(|c| c.is_small()) {
                *small_occurences.entry(c).or_insert(0) += 1;
            }

            if small_occurences.iter().any(|(_, count)| *count >= 2) {
                false
            } else {
                true
            }
        } else {
            true
        }
    } else {
        true
    }
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Cave {
    fn is_small(&self) -> bool {
        matches!(self, Cave::Small(_))
    }
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Cave> {
        match s {
            "start" => Ok(Cave::Start),
            "end" => Ok(Cave::End),
            s => {
                if s.chars().next().map_or(false, |s| s.is_uppercase()) {
                    Ok(Cave::Big(s.to_owned()))
                } else {
                    Ok(Cave::Small(s.to_owned()))
                }
            }
        }
    }
}
