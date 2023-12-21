pub fn p1(input: &str) -> String {
    let (plots, mut start, _) = parse_input(input);

    for _ in 0..64 {
        start = step_p1(&plots, start);
    }

    format!("Number of garden plots after 64 steps: {}", start.len())
}

pub fn p2(input: &str) -> String {
    let (plots, mut start, (w, h)) = parse_input(input);

    let mut ys = Vec::new();
    let xhat: i64 = 26501365;
    let rem = xhat.rem_euclid(w);
    let offset = 2;
    let x = (xhat - rem).div_euclid(w) - offset;

    for i in 1_i64..=6400 {
        start = step_p2(&plots, start, (w, h));

        if i.rem_euclid(w) == rem && i >= rem + offset * w {
            ys.push(start.len() as i64);

            if ys.len() == 3 {
                let val = lagrange_extrapolate_2deg(ys[0], ys[1], ys[2], x);

                return format!("Number of garden plots: {val}");
            }
        }
    }

    panic!("Noope")
}

fn parse_input(input: &str) -> (HashSet<Pos>, HashSet<Pos>, (i64, i64)) {
    let (_rocks, plots, start) = util::char_grid_iter(input).fold(
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

    let (w, h) = util::grid_dimensions(input);
    let w = w as i64;
    let h = h as i64;

    (plots, start, (w, h))
}

fn step_p1(plots: &HashSet<Pos>, reachable: impl IntoIterator<Item = Pos>) -> HashSet<Pos> {
    let r: HashSet<Pos> = reachable
        .into_iter()
        .flat_map(|pos| [pos + NORTH, pos + SOUTH, pos + EAST, pos + WEST])
        .collect();

    r.intersection(plots).cloned().collect()
}

fn step_p2(
    plots: &HashSet<Pos>,
    reachable: impl IntoIterator<Item = Pos>,
    (w, h): (i64, i64),
) -> HashSet<Pos> {
    let r: HashSet<Pos> = reachable
        .into_iter()
        .flat_map(|pos| [pos + NORTH, pos + SOUTH, pos + EAST, pos + WEST])
        .collect();

    r.into_iter()
        .filter(|pos| {
            let p2 = Pos::new(pos.re.rem_euclid(w), pos.im.rem_euclid(h));

            plots.contains(&p2)
        })
        .collect()
}

fn lagrange_extrapolate_2deg(y0: i64, y1: i64, y2: i64, x: i64) -> i64 {
    y0 * (x - 1) * (x - 2) / 2 - y1 * x * (x - 2) + y2 * x * (x - 1) / 2
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
