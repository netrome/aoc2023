pub fn p1(input: &str) -> String {
    let plan: DigPlan = input.parse().unwrap();
    let trench = plan.loop_of_trench();
    let trench_set = trench.iter().cloned().collect();

    let dim = Dim::infer(trench);
    let dim = dim.expand();
    let outside = fill(dim.bottom_left, &dim, &trench_set);

    let lagoon_area = dim.area() as usize - outside.len();

    format!("Interior volume: {}", lagoon_area)
}

pub fn p2(input: &str) -> String {
    let plan: DigPlan = input.parse().unwrap();

    let l = plan.loop_edges(true);
    let b = plan.border_size(true);

    let a = shoelace_area(l.clone()).abs() + b / 2 + 1;

    format!("Area: {}", a)
}

fn shoelace_area(mut polygon: Vec<Pos>) -> i64 {
    polygon.push(*polygon.first().unwrap());

    polygon
        .windows(2)
        .map(|window| {
            let first = window[0];
            let second = window[1];

            first.re * second.im - first.im * second.re
        })
        .sum::<i64>()
        / 2
}

fn fill(point: Pos, dim: &Dim, border: &HashSet<Pos>) -> HashSet<Pos> {
    let mut q = vec![point];
    let mut res = HashSet::new();
    res.insert(point);

    while let Some(pos) = q.pop() {
        for pos2 in neighbors(&pos) {
            if dim.contains(pos2) && !border.contains(&pos2) && !res.contains(&pos2) {
                res.insert(pos2);
                q.push(pos2);
            }
        }
    }

    res
}

#[derive(Debug)]
struct DigPlan {
    instrs: Vec<Instr>,
}

impl DigPlan {
    fn loop_of_trench(&self) -> Vec<Pos> {
        self.instrs
            .iter()
            .fold(vec![Pos::new(0, 0)], |mut acc, instr| {
                let last_pos = *acc.last().expect("Impossibru");
                let dir = instr.dir.from_above();

                (0..instr.len as i64).for_each(|offset| acc.push(last_pos + (offset + 1) * dir));
                acc
            })
    }

    //fn loop_corners(&self, use_color: bool) -> Vec<Corner> {
    //    let mut v: Vec<_> = self
    //        .instrs
    //        .iter()
    //        .map(|instr| {
    //            if use_color {
    //                instr.color.to_instruction()
    //            } else {
    //                (instr.dir.from_above(), instr.len as i64)
    //            }
    //        })
    //        .collect();
    //    v.push(*v.first().expect("Crap"));

    //    let mut pos = Pos::new(0, 0);

    //    v.windows(2)
    //        .map(|w| {
    //            let first = w[0];
    //            let second = w[1];

    //            pos += first.0 * first.1;
    //            let angle = angle(first.0, second.0);

    //            Corner {
    //                pos: pos.clone(),
    //                dir: first.0,
    //                angle,
    //            }
    //        })
    //        .collect()
    //}

    fn loop_edges(&self, use_color: bool) -> Vec<Pos> {
        self.instrs
            .iter()
            .map(|instr| {
                if use_color {
                    instr.color.to_instruction()
                } else {
                    (instr.dir.from_above(), instr.len as i64)
                }
            })
            .fold(vec![Pos::new(0, 0)], |mut acc, (dir, len)| {
                let last_pos = *acc.last().expect("Impossibru");
                acc.push(last_pos + (dir * len));
                acc
            })
    }

    fn border_size(&self, use_color: bool) -> i64 {
        self.instrs
            .iter()
            .map(|instr| {
                if use_color {
                    instr.color.to_instruction().1
                } else {
                    instr.len as i64
                }
            })
            .sum()
    }
}

#[derive(Debug)]
struct Corner {
    pos: Pos,
    dir: Dir,
    angle: i64,
}

impl Corner {
    fn shoelace_pos(&self) -> Pos {
        todo!();
    }
}

impl FromStr for DigPlan {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instrs = s
            .trim()
            .lines()
            .map(|line| sscanf::scanf!(line.trim(), "{Instr}").unwrap())
            .collect();

        Ok(Self { instrs })
    }
}

#[derive(Debug, sscanf::FromScanf)]
#[sscanf(format = "{dir} {len} ({color})")]
struct Instr {
    dir: Direction,
    len: usize,
    color: Color,
}

#[derive(Debug, Clone, Copy, sscanf::FromScanf)]
enum Direction {
    #[sscanf(format = "U")]
    Up,
    #[sscanf(format = "D")]
    Down,
    #[sscanf(format = "L")]
    Left,
    #[sscanf(format = "R")]
    Right,
}

impl Direction {
    fn from_above(&self) -> Dir {
        match self {
            Self::Up => NORTH,
            Self::Down => SOUTH,
            Self::Left => WEST,
            Self::Right => EAST,
        }
    }
}

#[derive(Debug, Clone, Copy, sscanf::FromScanf)]
#[sscanf(format = "#{r:x}{g:x}{b:x}")]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn to_instruction(&self) -> (Dir, i64) {
        let len = (self.r as i64) * 2_i64.pow(12)
            + (self.g as i64) * 2_i64.pow(4)
            + (self.b as i64) / 2_i64.pow(4);
        let dir = match self.b % 16 {
            0 => EAST,
            1 => SOUTH,
            2 => WEST,
            3 => NORTH,
            _ => panic!("Oh nooooo"),
        };

        (dir, len)
    }
}

#[derive(Debug, Clone)]
struct Dim {
    bottom_left: Pos,
    top_right: Pos,
}

impl Dim {
    fn infer(positions: impl IntoIterator<Item = Pos>) -> Self {
        let mut it = positions.into_iter();
        let init = it.next().unwrap();

        it.fold(
            Self {
                bottom_left: init,
                top_right: init,
            },
            |acc, pos| acc.extend_to_fit(pos),
        )
    }

    fn extend_to_fit(mut self, pos: Pos) -> Self {
        self.bottom_left.im = self.bottom_left.im.min(pos.im);
        self.bottom_left.re = self.bottom_left.re.min(pos.re);

        self.top_right.im = self.top_right.im.max(pos.im);
        self.top_right.re = self.top_right.re.max(pos.re);

        self
    }

    fn contains(&self, pos: Pos) -> bool {
        self.bottom_left.im <= pos.im
            && pos.im <= self.top_right.im
            && self.bottom_left.re <= pos.re
            && pos.re <= self.top_right.re
    }

    fn expand(&self) -> Self {
        Self {
            bottom_left: self.bottom_left + Pos::new(-1, -1),
            top_right: self.top_right + Pos::new(1, 1),
        }
    }

    fn area(&self) -> i64 {
        let diff = self.top_right - self.bottom_left;
        (diff.re + 1) * (diff.im + 1)
    }
}

fn neighbors(point: &Pos) -> [Pos; 8] {
    [
        point + NORTH,
        point + NORTH + EAST,
        point + NORTH + WEST,
        point + EAST,
        point + WEST,
        point + SOUTH,
        point + SOUTH + EAST,
        point + SOUTH + WEST,
    ]
}

fn angle(v1: Dir, v2: Dir) -> i64 {
    let arg_as_float = |dir: Dir| Complex64::new(dir.re as f64, dir.im as f64).arg();
    let diff = arg_as_float(v2).to_degrees() as i64 - arg_as_float(v1).to_degrees() as i64;
    let adjusted = if diff < -180 {
        diff + 360
    } else if diff > 180 {
        diff - 360
    } else {
        diff
    };

    match adjusted {
        90 => adjusted,
        -90 => adjusted,
        _ => panic!("Unexpected angle"),
    }
}

type Pos = Complex<i64>;
type Dir = Complex<i64>;

static NORTH: Dir = Dir::new(0, 1);
static SOUTH: Dir = Dir::new(0, -1);
static EAST: Dir = Dir::new(1, 0);
static WEST: Dir = Dir::new(-1, 0);

use std::{collections::HashSet, str::FromStr};

use num::{
    complex::{Complex64, ComplexFloat},
    Complex, Float,
};

use crate::solution::Solution;
inventory::submit!(Solution::new(18, 1, p1));
inventory::submit!(Solution::new(18, 2, p2));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_to_instr() {
        test_color("#70c710", EAST, 461937);
        test_color("#59c680", EAST, 367720);
        test_color("#8ceee2", WEST, 577262);
        test_color("#caa173", NORTH, 829975);
        test_color("#411b91", SOUTH, 266681);
    }

    fn test_color(color: &str, expected_dir: Dir, expected_len: i64) {
        let color: Color = sscanf::scanf!(color, "{Color}").unwrap();
        let (dir, len) = color.to_instruction();

        assert_eq!(dir, expected_dir);
        assert_eq!(len, expected_len);
    }
}
