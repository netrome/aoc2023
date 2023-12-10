pub fn p1(input: &str) -> String {
    let maze = Maze::from_input(input);

    let dist = maze.distance_to_farthest_connecting_pipe_from_animal_entry();

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
    width: usize,
    height: usize,
    animal_entry: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pipe {
    connections: [Position; 2],
}

impl Maze {
    fn from_input(input: &str) -> Self {
        let mut pipes: HashMap<Position, Pipe> = input
            .trim()
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(im, line)| {
                line.trim().chars().enumerate().filter_map(move |(re, c)| {
                    c.try_into()
                        .ok()
                        .map(|pipe| (Position::new(re as i64, im as i64), pipe))
                })
            })
            .collect();

        let height = input.lines().count();
        let width = input.lines().next().unwrap().trim().len();

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
            height,
            width,
        }
    }

    fn distance_to_farthest_connecting_pipe_from_animal_entry(&self) -> usize {
        let entry_pipe = self.pipes.get(&self.animal_entry).expect("No entry pipe");

        let mut distance: usize = 0;
        let mut left = (self.animal_entry, entry_pipe.connections[0]);
        let mut right = (self.animal_entry, entry_pipe.connections[1]);

        loop {
            left.0 += left.1;
            right.0 += right.1;
            distance += 1;

            if left.0 == right.0 {
                break;
            }

            left.1 = self
                .pipes
                .get(&left.0)
                .expect("No pipe")
                .next_direction(left.1);

            right.1 = self
                .pipes
                .get(&right.0)
                .expect("No pipe")
                .next_direction(right.1);
        }

        distance
    }

    fn walk(&self) -> Vec<Position> {
        let entry_pipe = self.pipes.get(&self.animal_entry).expect("No entry pipe");

        let mut current = (self.animal_entry, entry_pipe.connections[0]);
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
        self.walk()
            .into_iter()
            .filter(|pos| self.pipes.get(pos).expect("No pipe").is_corner())
            .collect()
    }

    fn non_loop_tiles(&self) -> Vec<Position> {
        let main_loop: HashSet<Position> = self.walk().into_iter().collect();

        (0..self.width)
            .flat_map(|re| (0..self.height).map(move |im| Position::new(re as i64, im as i64)))
            .filter(|pos| !main_loop.contains(pos))
            .collect()
    }

    fn enclosed_tiles(&self) -> usize {
        let tiles = self.non_loop_tiles();
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
}

impl Pipe {
    fn new(first: Direction, second: Direction) -> Self {
        Self {
            connections: [first, second],
        }
    }

    fn animal_placeholder() -> Self {
        Self::new(NORTH, NORTH)
    }

    fn connects(&self, pipe: Position, target: Position) -> bool {
        self.connections.iter().any(|c| pipe + c == target)
    }

    fn next_direction(&self, dir: Direction) -> Direction {
        let [first, second] = self.connections;
        let opposite = dir * -1;

        match (opposite == first, opposite == second) {
            (true, false) => second,
            (false, true) => first,
            _ => panic!("One cannot simply walk here"),
        }
    }

    fn is_corner(&self) -> bool {
        (self.connections[0] * self.connections[1]).im != 0
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
