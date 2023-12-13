pub fn p1(input: &str) -> String {
    let patterns: Vec<Pattern> = input
        .trim()
        .split("\n\n")
        .map(|line| line.parse().unwrap())
        .collect();

    let sum: i64 = patterns
        .into_iter()
        .map(|pattern| pattern.summarize_p1())
        .sum();

    format!("Sum: {}", sum)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

struct Pattern {
    grid: HashMap<Pos, char>,
    width: usize,
    height: usize,
}

impl Pattern {
    fn summarize_p1(&self) -> i64 {
        self.find_vertical_reflection().unwrap_or_else(|| {
            (self.height as i64 - self.find_horizontal_reflection().expect("Oh no!")) * 100
        })
    }

    fn find_horizontal_reflection(&self) -> Option<i64> {
        for y in 1..self.height as i64 {
            let delta_lim = (self.height as i64 - y).min(y);

            if (0..delta_lim).all(move |delta| {
                let left = self.get_row(y - delta - 1);
                let right = self.get_row(y + delta);

                left == right
            }) {
                return Some(y);
            };
        }

        None
    }

    fn find_vertical_reflection(&self) -> Option<i64> {
        self.transpose().find_horizontal_reflection()
    }

    fn transpose(&self) -> Self {
        let grid = self
            .grid
            .iter()
            .map(|(pos, char)| (Pos::new(pos.im, pos.re), *char))
            .collect();

        let width = self.height;
        let height = self.width;

        Self {
            grid,
            width,
            height,
        }
    }

    fn get_row(&self, y: i64) -> Vec<char> {
        (0..self.width as i64)
            .map(|x| *self.grid.get(&Pos::new(x, y)).expect("Impossibru"))
            .collect()
    }

    fn get_col(&self, x: i64) -> Vec<char> {
        (0..self.height as i64)
            .map(|y| *self.grid.get(&Pos::new(x, y)).expect("Impossibru"))
            .collect()
    }
}

impl FromStr for Pattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let (width, height) = parse::grid_dimensions(s);
        let grid = parse::char_grid_iter(s)
            .map(|(x, y, c)| (Pos::new(x as i64, y as i64), c))
            .collect();

        Ok(Self {
            grid,
            width,
            height,
        })
    }
}

type Pos = num::Complex<i64>;

use std::{collections::HashMap, str::FromStr};

use crate::{parse, solution::Solution};
inventory::submit!(Solution::new(13, 1, p1));
inventory::submit!(Solution::new(13, 2, p2));
