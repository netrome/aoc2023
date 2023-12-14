pub fn p1(input: &str) -> String {
    let mut dish: Dish = input.parse().unwrap();
    dish.rounded = dish.tilt_north();

    format!("Total load: {}", dish.total_load())
}

pub fn p2(input: &str) -> String {
    let dish: Dish = input.parse().unwrap();

    let cycle = Cycle::find(|| {
        dish.clone()
            .loads_per_cycle()
            .scan(VecDeque::from(vec![0, 0, 0]), |acc, val| {
                acc.push_back(val);
                acc.pop_front();
                Some(acc.clone())
            })
    });

    let load = cycle.evaluate(dish.loads_per_cycle(), 999999999);

    format!("Total load after a billion cycles: {}", load)
}

#[derive(Clone)]
struct Dish {
    squares: HashSet<Pos>,
    rounded: Vec<Pos>,
    size: Pos,
}

impl Dish {
    fn loads_per_cycle(mut self) -> impl Iterator<Item = i64> {
        (0..).map(move |_| {
            self.tilt_cycle();
            self.total_load()
        })
    }

    fn tilt_cycle(&mut self) {
        self.rounded = self.tilt_north();
        self.rounded = self.tilt_west();
        self.rounded = self.tilt_south();
        self.rounded = self.tilt_east();
    }

    fn total_load(&self) -> i64 {
        self.rounded.iter().map(|pos| pos.im + 1).sum()
    }

    fn tilt_north(&self) -> Vec<Pos> {
        (0..self.size.re)
            .flat_map(|x| {
                let start = Pos::new(x, 0);
                let stop = Pos::new(x, self.size.im);

                self.tilt1d(start, stop, NORTH)
            })
            .collect()
    }

    fn tilt_south(&self) -> Vec<Pos> {
        (0..self.size.re)
            .flat_map(|x| {
                let start = Pos::new(x, self.size.im - 1);
                let stop = Pos::new(x, -1);

                self.tilt1d(start, stop, SOUTH)
            })
            .collect()
    }

    fn tilt_east(&self) -> Vec<Pos> {
        (0..self.size.im)
            .flat_map(|y| {
                let start = Pos::new(0, y);
                let stop = Pos::new(self.size.re, y);

                self.tilt1d(start, stop, EAST)
            })
            .collect()
    }

    fn tilt_west(&self) -> Vec<Pos> {
        (0..self.size.im)
            .flat_map(|y| {
                let start = Pos::new(self.size.re - 1, y);
                let stop = Pos::new(-1, y);

                self.tilt1d(start, stop, WEST)
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

#[derive(Debug)]
struct Cycle {
    entry: usize,
    len: usize,
}

impl Cycle {
    fn find<T: PartialEq, It: IntoIterator<Item = T>>(create_iterator: impl Fn() -> It) -> Self {
        let tortoise = create_iterator().into_iter();
        let hare = create_iterator().into_iter().skip(1).step_by(2);

        let mut intersections = tortoise
            .zip(hare)
            .enumerate()
            .filter(|(_, (t, h))| t == h)
            .take(2);

        let entry = intersections.next().unwrap().0;
        let len = intersections.next().unwrap().0 - entry;

        Self { entry, len }
    }

    fn evaluate(&self, it: impl IntoIterator<Item = i64>, at: usize) -> i64 {
        let cycle_idx = at.checked_sub(self.entry).unwrap() % self.len;

        it.into_iter().skip(self.entry + cycle_idx).next().unwrap()
    }
}

impl FromStr for Dish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (squares, rounded) = util::char_grid_iter(s)
            .map(|(x, y, c)| (Pos::new(x as i64, y as i64), c))
            .filter(|(_, c)| *c != '.')
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
    collections::{HashSet, VecDeque},
    str::FromStr,
};

type Pos = Complex<i64>;
type Dir = Complex<i64>;

static NORTH: Dir = Dir::new(0, 1);
static SOUTH: Dir = Dir::new(0, -1);
static EAST: Dir = Dir::new(1, 0);
static WEST: Dir = Dir::new(-1, 0);

use num::Complex;

use crate::{solution::Solution, util};
inventory::submit!(Solution::new(14, 1, p1));
inventory::submit!(Solution::new(14, 2, p2));
