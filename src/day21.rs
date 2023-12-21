pub fn p1(input: &str) -> String {
    let (_rocks, plots, mut start) = util::char_grid_iter(input).fold(
        (HashSet::new(), HashSet::new(), HashSet::new()),
        |mut acc, (x, y, c)| {
            let pos = Pos::new(x as i64, y as i64);

            match c {
                '#' => {
                    acc.0.insert(pos);
                }
                '.' => {
                    acc.1.insert(pos);
                }
                'S' => {
                    acc.1.insert(pos);
                    acc.2.insert(pos);
                }
                _ => panic!("Unexpected char"),
            };

            acc
        },
    );

    for _ in 0..64 {
        start = step_p1(&plots, start);
    }

    format!("Number of garden plots after 64 steps: {}", start.len())
}

pub fn p2(_input: &str) -> String {
    todo!();
}

fn step_p1(plots: &HashSet<Pos>, reachable: impl IntoIterator<Item = Pos>) -> HashSet<Pos> {
    let r: HashSet<Pos> = reachable
        .into_iter()
        .flat_map(|pos| [pos + NORTH, pos + SOUTH, pos + EAST, pos + WEST])
        .collect();

    r.intersection(plots).cloned().collect()
}

type Pos = Complex<i64>;
type Dir = Complex<i64>;
static NORTH: Dir = Dir::new(0, 1);
static SOUTH: Dir = Dir::new(0, -1);
static EAST: Dir = Dir::new(1, 0);
static WEST: Dir = Dir::new(-1, 0);

use std::collections::HashSet;

use num::Complex;

use crate::{solution::Solution, util};
inventory::submit!(Solution::new(21, 1, p1));
inventory::submit!(Solution::new(21, 2, p2));
