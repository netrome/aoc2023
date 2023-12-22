pub fn p1(input: &str) -> String {
    let mut bricks = parse_input(input);
    let mut world: HashMap<Pos, usize> = bricks
        .iter()
        .flat_map(|brick| brick.positions().into_iter().map(move |p| (p, brick.id)))
        .collect();

    while bricks.iter_mut().any(|b| b.try_move(&mut world, -1 * Z)) {}

    let count = bricks
        .iter()
        .filter(|brick| {
            brick.neighbors(&world, Z).into_iter().all(|neighbor_id| {
                bricks
                    .get(neighbor_id)
                    .unwrap()
                    .neighbors(&world, -1 * Z)
                    .len()
                    > 1
            })
        })
        .count();

    format!("Count: {:?}", count)
}

pub fn p2(input: &str) -> String {
    let mut bricks = parse_input(input);
    let mut world: HashMap<Pos, usize> = bricks
        .iter()
        .flat_map(|brick| brick.positions().into_iter().map(move |p| (p, brick.id)))
        .collect();

    while bricks.iter_mut().any(|b| b.try_move(&mut world, -1 * Z)) {}
    todo!();
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let (start, stop) = sscanf::scanf!(line.trim(), "{Pos}~{Pos}").unwrap();
            Brick { start, stop, id }
        })
        .collect()
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Brick {
    id: usize,
    start: Pos,
    stop: Pos,
}

impl Brick {
    fn positions(&self) -> Vec<Pos> {
        let mut cursor = self.start;
        let dir = (self.stop - self.start).axis();
        let mut res = vec![self.start];

        while cursor != self.stop {
            cursor = cursor + dir;
            res.push(cursor);
        }

        res
    }

    fn neighbors(&self, world: &HashMap<Pos, usize>, delta: Pos) -> HashSet<usize> {
        self.positions()
            .into_iter()
            .flat_map(|pos| world.get(&(pos + delta)))
            .cloned()
            .filter(|id| *id != self.id)
            .collect()
    }

    fn try_move(&mut self, world: &mut HashMap<Pos, usize>, delta: Pos) -> bool {
        let positions = self.positions();
        let new_positions: Vec<Pos> = positions.iter().map(|pos| *pos + delta).collect();

        if !new_positions
            .iter()
            .any(|pos| pos.z < 1 || world.get(pos).map(|id| *id != self.id).unwrap_or(false))
        {
            for pos in positions {
                world.remove(&pos);
            }

            for pos in new_positions {
                world.insert(pos, self.id);
            }

            self.start = self.start + delta;
            self.stop = self.stop + delta;

            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, sscanf::FromScanf)]
#[sscanf(format = "{x},{y},{z}")]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    const fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn axis(&self) -> Pos {
        match (self.x, self.y, self.z) {
            (x, 0, 0) => x.signum() * X,
            (0, y, 0) => y.signum() * Y,
            (0, 0, z) => z.signum() * Z,
            _ => panic!("Not axis"),
        }
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;

        self
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;

        self
    }
}

impl std::ops::Mul<i64> for Pos {
    type Output = Self;

    fn mul(mut self, rhs: i64) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}

impl std::ops::Mul<Pos> for i64 {
    type Output = Pos;

    fn mul(self, rhs: Pos) -> Self::Output {
        rhs * self
    }
}

static X: Pos = Pos::new(1, 0, 0);
static Y: Pos = Pos::new(0, 1, 0);
static Z: Pos = Pos::new(0, 0, 1);

use std::collections::{HashMap, HashSet};

use crate::solution::Solution;
inventory::submit!(Solution::new(22, 1, p1));
inventory::submit!(Solution::new(22, 2, p2));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pos_works() {
        let p1 = Pos::new(4, 3, 123);
        let p2 = Pos::new(2, 3, 123);
    }

    #[test]
    fn brick_works() {
        let start = Pos::new(4, 3, 123);
        let stop = Pos::new(4, 3, 123);
        let id = 1;

        let b = Brick { start, stop, id };

        assert_eq!(b.positions().len(), 1);
    }
}
