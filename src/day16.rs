pub fn p1(input: &str) -> String {
    let grid: Grid = util::char_grid_iter(input)
        .map(|(x, y, c)| (Pos::new(x as i64, y as i64), c))
        .collect();

    let (width, height) = util::grid_dimensions(input);

    let mut seen = HashSet::new();
    seen.insert(Beam::start(height as i64));

    let mut beams = vec![Beam::start(height as i64)];

    while let Some(beam) = beams.pop() {
        for next in beam.advance(&grid).into_iter().filter_map(|n| n) {
            if next.within_contraption(width as i64, height as i64) && !seen.contains(&next) {
                seen.insert(next.clone());
                beams.push(next);
            }
        }
    }

    let energized: HashSet<Pos> = seen.into_iter().map(|beam| beam.pos).collect();

    format!("Energized tiles: {}", energized.len())
}

pub fn p2(_input: &str) -> String {
    todo!();
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Beam {
    pos: Pos,
    dir: Dir,
}

impl Beam {
    fn new(pos: Pos, dir: Dir) -> Self {
        Self { pos, dir }
    }

    fn start(height: i64) -> Self {
        let pos = Pos::new(0, height as i64 - 1);
        let dir = EAST;

        Self { pos, dir }
    }

    fn advance(&self, grid: &Grid) -> [Option<Self>; 2] {
        encounter(self.dir, grid.get(&self.pos).cloned().unwrap())
            .map(|maybe_dir| maybe_dir.map(|dir| Self::new(self.pos + dir, dir)))
    }

    fn within_contraption(&self, width: i64, height: i64) -> bool {
        self.pos.re >= 0 && self.pos.re < width && self.pos.im >= 0 && self.pos.im < height
    }
}

fn encounter(beam: Dir, tile: char) -> [Option<Dir>; 2] {
    match tile {
        '.' => [Some(beam), None],
        '-' => split([EAST, WEST], beam),
        '|' => split([NORTH, SOUTH], beam),
        '/' => [Some(mirror(1, beam)), None],
        '\\' => [Some(mirror(-1, beam)), None],
        _ => panic!("Unhandled case"),
    }
}

fn split(directions: [Dir; 2], beam: Dir) -> [Option<Dir>; 2] {
    if directions.contains(&beam) {
        [Some(beam), None]
    } else {
        directions.map(|dir| Some(dir))
    }
}

fn mirror(sign: i64, beam: Dir) -> Dir {
    Dir::new(sign * beam.im, sign * beam.re)
}

type Grid = HashMap<Pos, char>;

type Pos = Complex<i64>;
type Dir = Complex<i64>;

static NORTH: Dir = Dir::new(0, 1);
static SOUTH: Dir = Dir::new(0, -1);
static EAST: Dir = Dir::new(1, 0);
static WEST: Dir = Dir::new(-1, 0);

use std::collections::{HashMap, HashSet};

use num::Complex;

use crate::{solution::Solution, util};
inventory::submit!(Solution::new(16, 1, p1));
inventory::submit!(Solution::new(16, 2, p2));
