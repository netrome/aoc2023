pub fn p1(input: &str) -> String {
    let dish: Dish = input.parse().unwrap();
    let sum: i64 = dish.tilt_north().into_iter().map(|pos| pos.im + 1).sum();

    format!("Total load: {}", sum)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

struct Dish {
    squares: HashSet<Pos>,
    rounded: Vec<Pos>,
    size: Pos,
}

impl Dish {
    fn tilt_north(&self) -> Vec<Pos> {
        (0..self.size.re)
            .flat_map(|x| {
                let start = Pos::new(x, 0);
                let stop = Pos::new(x, self.size.im);

                self.tilt1d(start, stop, NORTH)
            })
            .collect()
    }

    fn tilt1d(&self, start: Pos, stop: Pos, dir: Dir) -> Vec<Pos> {
        let mut cursor = start;

        let mut count: i64 = 0;
        let mut acc = Vec::new();

        while cursor != stop {
            if self.rounded.contains(&cursor) {
                count += 1;
            }

            if self.is_square(cursor) {
                for offset in 1..=count {
                    acc.push(cursor - dir * offset)
                }

                count = 0;
            }

            cursor += dir;
        }

        for offset in 1..=count {
            acc.push(cursor - dir * offset)
        }

        acc
    }

    fn is_square(&self, pos: Pos) -> bool {
        self.squares.contains(&pos)
    }
}

impl FromStr for Dish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (squares, rounded) = util::char_grid_iter(s)
            .map(|(x, y, c)| (Pos::new(x as i64, y as i64), c))
            .filter(|(pos, c)| *c != '.')
            .fold((HashSet::new(), Vec::new()), |mut acc, item| match item.1 {
                '#' => {
                    acc.0.insert(item.0);
                    acc
                }
                'O' => {
                    acc.1.push(item.0);
                    acc
                }
                _ => panic!("Got unexpected char"),
            });

        let (width, height) = util::grid_dimensions(s);

        Ok(Self {
            squares,
            rounded,
            size: Pos::new(width as i64, height as i64),
        })
    }
}

use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

type Pos = Complex<i64>;
type Dir = Complex<i64>;

static NORTH: Dir = Dir::new(0, 1);
static SOUTH: Dir = Dir::new(0, -1);
static EAST: Dir = Dir::new(1, 0);
static WEST: Dir = Dir::new(-1, 0);

fn project(pos: Pos, dir: Dir) -> i64 {
    pos.re * dir.re + pos.im * dir.im
}

use num::{Complex, Signed};

use crate::{solution::Solution, util};
inventory::submit!(Solution::new(14, 1, p1));
inventory::submit!(Solution::new(14, 2, p2));
