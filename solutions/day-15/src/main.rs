use ndarray::Array2;
use pathfinding::directed::astar::astar;

const INPUT: &str = include_str!("../../../inputs/day-15.txt");

fn main() -> anyhow::Result<()> {
    let width = INPUT.lines().next().unwrap().len();
    let height = INPUT.lines().count();

    let mut map = Array2::default((height, width));
    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[(y, x)] = (c as u8) - 48;
        }
    }

    let answer = shortest_path_cost(&map);
    println!("first answer is {}", answer);

    let answer = shortest_path_cost(&Map5x5 { base: map });
    println!("second answer is {}", answer);

    Ok(())
}

fn shortest_path_cost<M: RiskLevel>(map: &M) -> usize {
    let (height, width) = map.dimensions();

    let (_, cost) = astar(
        &(0usize, 0usize),
        |&(y, x)| {
            let left = map.risk((y, x - 1)).map(|c| ((y, x - 1), c as usize));
            let right = map.risk((y, x + 1)).map(|c| ((y, x + 1), c as usize));
            let bottom = map.risk((y + 1, x)).map(|c| ((y + 1, x), c as usize));
            let top = map
                .risk((y.wrapping_sub(1), x))
                .map(|c| ((y - 1, x), c as usize));

            [left, right, bottom, top]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
        },
        |&(y, x)| abs_sub(height - 1, y) + abs_sub(width - 1, x),
        |&pos| pos == (height - 1, width - 1),
    )
    .unwrap();

    cost
}

fn abs_sub(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

trait RiskLevel {
    fn dimensions(&self) -> (usize, usize);
    fn risk(&self, p: (usize, usize)) -> Option<u8>;
}

impl RiskLevel for Array2<u8> {
    fn dimensions(&self) -> (usize, usize) {
        self.dim()
    }

    fn risk(&self, p: (usize, usize)) -> Option<u8> {
        self.get(p).copied()
    }
}

struct Map5x5<M> {
    base: M,
}

impl<M: RiskLevel> RiskLevel for Map5x5<M> {
    fn dimensions(&self) -> (usize, usize) {
        let (height, width) = self.base.dimensions();
        (height * 5, width * 5)
    }

    fn risk(&self, (y, x): (usize, usize)) -> Option<u8> {
        let (base_height, base_width) = self.base.dimensions();
        let (height, width) = self.dimensions();
        if y >= height || x >= width {
            None
        } else {
            let dist = (y / base_height) + (x / base_width);
            let y = y % base_height;
            let x = x % base_width;
            self.base
                .risk((y, x))
                .map(|c| ((c as usize - 1 + dist) % 9 + 1) as u8)
        }
    }
}
