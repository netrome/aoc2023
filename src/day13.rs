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

pub fn p2(input: &str) -> String {
    let patterns: Vec<Pattern> = input
        .trim()
        .split("\n\n")
        .map(|line| line.parse().unwrap())
        .collect();

    let sum: i64 = patterns
        .into_iter()
        .map(|mut pattern| pattern.summarize_p2())
        .sum();

    format!("Sum: {}", sum)
}

struct Pattern {
    grid: HashMap<Pos, char>,
    width: usize,
    height: usize,
}

impl std::fmt::Debug for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = (0..self.height)
            .map(|y| {
                (0..self.width)
                    .map(|x| self.grid.get(&Pos::new(x as i64, y as i64)).unwrap())
                    .collect()
            })
            .collect();

        let lines: Vec<String> = lines.into_iter().rev().collect();
        f.write_fmt(format_args!("===\n{}\n===", lines.join("\n")))
    }
}

impl Pattern {
    fn summarize_p1(&self) -> i64 {
        self.try_summarize().next().unwrap()
    }

    fn summarize_p2(&mut self) -> i64 {
        let p1_val = self.try_summarize().next().unwrap();

        for x in 0..self.width as i64 {
            for y in 0..self.height as i64 {
                let pos = Pos::new(x, y);
                self.smudge(pos);
                let maybe_summary = self.try_summarize().filter(|v| *v != p1_val).next();
                if let Some(v) = maybe_summary {
                    return v;
                } else {
                    self.smudge(pos);
                }
            }
        }

        panic!("We should not get here!")
    }

    fn smudge(&mut self, p: Pos) {
        let new = match *self.grid.get(&p).expect("Waaat?") {
            '.' => '#',
            '#' => '.',
            _ => panic!("Oh crap!"),
        };

        self.grid.insert(p, new);
    }

    fn try_summarize(&self) -> impl Iterator<Item = i64> + '_ {
        self.find_vertical_reflections().into_iter().chain(
            self.find_horizontal_reflections()
                .into_iter()
                .map(|v| (self.height as i64 - v) * 100),
        )
    }

    fn find_horizontal_reflections(&self) -> impl Iterator<Item = i64> + '_ {
        (1..self.height as i64).filter(move |y| {
            let delta_lim = (self.height as i64 - y).min(*y);

            (0..delta_lim).all(move |delta| {
                let left = self.get_row(y - delta - 1);
                let right = self.get_row(y + delta);

                left == right
            }) && delta_lim > 0
        })
    }

    fn find_vertical_reflections(&self) -> impl Iterator<Item = i64> + '_ {
        (1..self.width as i64).filter(move |x| {
            let delta_lim = (self.width as i64 - x).min(*x);

            (0..delta_lim).all(move |delta| {
                let left = self.get_col(x - delta - 1);
                let right = self.get_col(x + delta);

                left == right
            }) && delta_lim > 0
        })
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
