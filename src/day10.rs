pub fn p1(input: &str) -> String {
    let maze = Maze::from_input(input);

    let dist = maze.farthest_distance();

    format!("Distance: {:?}", dist)
}

pub fn p2(input: &str) -> String {
    let maze = Maze::from_input(input);

    let n = maze.enclosed_tiles();

    format!("Enclosed tiles: {:?}", n)
}

#[derive(Debug)]
struct Maze {
    pipes: HashMap<Position, Pipe>,
    animal_entry: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pipe {
    a: Position,
    b: Position,
}

impl Maze {
    fn from_input(input: &str) -> Self {
        let mut pipes: HashMap<Position, Pipe> = crate::util::char_grid_iter(input)
            .map(|(x, y, pipe)| (Position::new(x as i64, y as i64), pipe))
            .collect();

        let animal_entry = *pipes
            .iter()
            .find(|(_, pipe)| **pipe == Pipe::animal_placeholder())
            .expect("No starting pos found")
            .0;

        let animal_directions: Vec<Position> = DIRECTIONS
            .iter()
            .map(|d| (d, animal_entry + d))
            .filter(|(_d, pos)| {
                pipes
                    .get(pos)
                    .map(|pipe| pipe.connects(*pos, animal_entry))
                    .unwrap_or(false)
            })
            .map(|(d, _)| *d)
            .collect();

        assert_eq!(animal_directions.len(), 2);

        pipes.insert(
            animal_entry,
            Pipe::new(animal_directions[0], animal_directions[1]),
        );

        Self {
            pipes,
            animal_entry,
        }
    }

    fn farthest_distance(&self) -> usize {
        self.walk(false)
            .into_iter()
            .zip(self.walk(true))
            .skip(1)
            .take_while(|(left, right)| left != right)
            .count()
            + 1
    }

    fn enclosed_tiles(&self) -> usize {
        let tiles = self.enclosed_tile_candidates();
        let mut rotations = vec![0.0; tiles.len()];

        for pair in self.corner_walk().windows(2) {
            for (idx, tile) in tiles.iter().enumerate() {
                let v2 = pair[1] - tile;
                let v1 = pair[0] - tile;

                rotations[idx] += angle(v1, v2);
            }
        }

        rotations.into_iter().filter(|rot| rot.abs() > PI).count()
    }

    fn walk(&self, reverse: bool) -> Vec<Position> {
        let entry_pipe = self.pipes.get(&self.animal_entry).expect("No entry pipe");

        let direction = if reverse { entry_pipe.a } else { entry_pipe.b };

        let mut current = (self.animal_entry, direction);
        let mut positions = vec![self.animal_entry];

        loop {
            current.0 += current.1;
            current.1 = self
                .pipes
                .get(&current.0)
                .expect("No pipe")
                .next_direction(current.1);

            positions.push(current.0);

            if current.0 == self.animal_entry {
                break;
            }
        }

        positions
    }

    fn corner_walk(&self) -> Vec<Position> {
        self.walk(false)
            .into_iter()
            .filter(|pos| self.pipes.get(pos).expect("No pipe").is_corner())
            .collect()
    }

    fn enclosed_tile_candidates(&self) -> Vec<Position> {
        let main_loop: HashSet<Position> = self.walk(false).into_iter().collect();

        let (x, y) = self
            .corner_walk()
            .into_iter()
            .fold((Vec::new(), Vec::new()), |mut acc, c| {
                acc.0.push(c.re);
                acc.1.push(c.im);
                acc
            });

        let min_x = x.iter().min().unwrap() + 1;
        let max_x = x.iter().max().unwrap();

        let min_y = y.iter().min().unwrap() + 1;
        let max_y = y.iter().max().unwrap();

        (min_x..*max_x)
            .flat_map(|re| (min_y..*max_y).map(move |im| Position::new(re, im)))
            .filter(|pos| !main_loop.contains(pos))
            .collect()
    }
}

impl Pipe {
    fn new(a: Direction, b: Direction) -> Self {
        Self { a, b }
    }

    fn animal_placeholder() -> Self {
        Self::new(NORTH, NORTH)
    }

    fn connects(&self, pipe: Position, target: Position) -> bool {
        pipe + self.a == target || pipe + self.b == target
    }

    fn next_direction(&self, dir: Direction) -> Direction {
        let opposite = dir * -1;

        match (opposite == self.a, opposite == self.b) {
            (true, false) => self.b,
            (false, true) => self.a,
            _ => panic!("One cannot simply walk here"),
        }
    }

    fn is_corner(&self) -> bool {
        (self.a * self.b).im != 0
    }
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Pipe::new(NORTH, SOUTH),
            '-' => Pipe::new(EAST, WEST),
            'L' => Pipe::new(NORTH, EAST),
            'J' => Pipe::new(NORTH, WEST),
            '7' => Pipe::new(SOUTH, WEST),
            'F' => Pipe::new(SOUTH, EAST),
            'S' => Pipe::animal_placeholder(),
            _ => anyhow::bail!("Not a pipe"),
        })
    }
}

type Position = Complex<i64>;
type Direction = Complex<i64>;
static NORTH: Direction = Direction::new(0, 1);
static SOUTH: Direction = Direction::new(0, -1);
static EAST: Direction = Direction::new(1, 0);
static WEST: Direction = Direction::new(-1, 0);
static DIRECTIONS: [Direction; 4] = [NORTH, SOUTH, EAST, WEST];

fn arg_as_float(pos: Position) -> f64 {
    let pos_as_float = Complex64::new(pos.re as f64, pos.im as f64);
    pos_as_float.arg()
}

fn angle(v1: Position, v2: Position) -> f64 {
    let diff = arg_as_float(v2) - arg_as_float(v1);

    if diff < -PI {
        diff + 2. * PI
    } else if diff > PI {
        diff - 2. * PI
    } else {
        diff
    }
}

use std::{
    collections::{HashMap, HashSet},
    f64::consts::PI,
};

use num::{complex::Complex64, Complex};

use crate::solution::Solution;
inventory::submit!(Solution::new(10, 1, p1));
inventory::submit!(Solution::new(10, 2, p2));
