use ndarray::Array2;
use std::fmt;

const INPUT: &str = include_str!("../../../inputs/day-11.txt");

fn main() -> anyhow::Result<()> {
    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();

    let mut map = Array2::default((height, width));
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let n = (c as u8 - b'0') as u32;
            map[(y, x)] = EnergyState::PreFlash(n);
        }
    }

    let mut flashes = 0;
    for _step in 0.. {
        let flash = increase_energies(&mut map);
        flashes += flash;

        if flash == width * height {
            println!("synchronize at {}", _step);
            break;
        }

        for y in 0..height {
            for x in 0..width {
                recursive_flash(&mut map, y, x);
            }
        }
    }

    let answer = flashes;
    println!("first answer is {}", answer);

    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();

    let mut map = Array2::default((height, width));
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let n = (c as u8 - b'0') as u32;
            map[(y, x)] = EnergyState::PreFlash(n);
        }
    }

    for step in 0.. {
        let flash_count = increase_energies(&mut map);

        if flash_count == width * height {
            println!("second answer is {}", step);
            break;
        }

        for y in 0..height {
            for x in 0..width {
                recursive_flash(&mut map, y, x);
            }
        }
    }

    Ok(())
}

fn increase_energies(map: &mut Array2<EnergyState>) -> usize {
    let mut flashes = 0;
    for e in map.iter_mut() {
        *e = match e {
            EnergyState::PreFlash(n) => EnergyState::PreFlash(*n + 1),
            EnergyState::Flashed(_) => {
                flashes += 1;
                EnergyState::PreFlash(1)
            }
        }
    }
    flashes
}

fn recursive_flash(map: &mut Array2<EnergyState>, y: usize, x: usize) {
    // flash only if energy > 9
    if map.get((y, x)).map_or(false, EnergyState::must_flash) {
        map[(y, x)].flash();
        for ym in -1..=1 {
            for xm in -1..=1 {
                if !(ym == 0 && xm == 0) {
                    let y = checked_add_signed(y, ym);
                    let x = checked_add_signed(x, xm);
                    if let Some((y, x)) = y.zip(x) {
                        if let Some(e) = map.get_mut((y, x)) {
                            e.increase();
                            recursive_flash(map, y, x);
                        }
                    }
                }
            }
        }
    }
}

/// <https://doc.rust-lang.org/std/primitive.usize.html#method.checked_add_signed>
fn checked_add_signed(x: usize, add: isize) -> Option<usize> {
    if add < 0 {
        x.checked_sub((-add) as usize)
    } else {
        x.checked_add(add as usize)
    }
}

#[derive(Copy, Clone)]
enum EnergyState {
    PreFlash(u32),
    Flashed(u32),
}

impl EnergyState {
    fn must_flash(&self) -> bool {
        matches!(self, EnergyState::PreFlash(n) if *n > 9)
    }

    fn increase(&mut self) {
        match self {
            EnergyState::PreFlash(n) => *n += 1,
            EnergyState::Flashed(n) => *n += 1,
        }
    }

    fn flash(&mut self) {
        *self = match self {
            EnergyState::PreFlash(n) => EnergyState::Flashed(*n),
            EnergyState::Flashed(_) => panic!("forbidden flash"),
        };
    }
}

impl Default for EnergyState {
    fn default() -> EnergyState {
        EnergyState::PreFlash(0)
    }
}

impl fmt::Debug for EnergyState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnergyState::PreFlash(n) => write!(fmt, "{}", n),
            EnergyState::Flashed(_) => write!(fmt, "0"),
        }
    }
}
