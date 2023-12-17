pub fn p1(input: &str) -> String {
    solve::<BasicCrucible>(input)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

fn solve<C: Crucible>(input: &str) -> String {
    let city: City = util::char_grid_iter(input)
        .map(|(x, y, c): (_, _, char)| (Pos::new(x as i64, y as i64), c.to_digit(10).unwrap()))
        .collect();

    let (width, height) = util::grid_dimensions(input);
    let width = width as i64;
    let height = height as i64;

    let mut distances: HashMap<C, u32> = HashMap::new();

    let start1 = C::new(lava_pool(height), EAST);
    let start2 = C::new(lava_pool(height), SOUTH);

    distances.insert(start1.clone(), 0);
    distances.insert(start2.clone(), 0);

    let mut heap: BinaryHeap<Reverse<(u32, C)>> = BinaryHeap::new();
    heap.push(Reverse((0, start1)));
    heap.push(Reverse((0, start2)));

    while let Some(Reverse((dist, crucible))) = heap.pop() {
        for c2 in crucible
            .advance(width, height)
            .into_iter()
            .filter_map(|c| c)
        {
            if !distances.contains_key(&c2) {
                let heat_loss = city.get(c2.pos()).expect("Impossibru");
                let d2 = dist + heat_loss;
                distances.insert(c2.clone(), d2);
                heap.push(Reverse((d2, c2)))
            }
        }
    }

    let end1 = C::new(factory(height), EAST);
    let end2 = C::new(factory(width), SOUTH);

    let d1 = distances.get(&end1).unwrap_or(&u32::MAX);
    let d2 = distances.get(&end2).unwrap_or(&u32::MAX);

    format!("Heat loss: {}", d1.min(d2))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BasicCrucible {
    pos: Pos,
    dir: Dir,
    line_len: usize,
}

trait Crucible: Sized + Ord + Clone + std::hash::Hash {
    fn new(pos: Pos, dir: Dir) -> Self;
    fn advance(self, width: i64, height: i64) -> [Option<Self>; 3];
    fn pos(&self) -> &Pos;
}

impl Crucible for BasicCrucible {
    fn new(pos: Pos, dir: Dir) -> Self {
        Self {
            pos,
            dir,
            line_len: 0,
        }
    }

    fn advance(self, width: i64, height: i64) -> [Option<Self>; 3] {
        let left_dir = self.dir * NORTH;
        let right_dir = self.dir * SOUTH;

        let left = Self::new(self.pos + left_dir, left_dir);
        let right = Self::new(self.pos + right_dir, right_dir);

        let mut forward = self;
        forward.pos += forward.dir;
        forward.line_len += 1;

        let left = if within_grid(left.pos, height, width) {
            Some(left)
        } else {
            None
        };

        let right = if within_grid(right.pos, height, width) {
            Some(right)
        } else {
            None
        };

        let forward = if within_grid(forward.pos, height, width) && forward.line_len < 3 {
            Some(forward)
        } else {
            None
        };

        [forward, left, right]
    }

    fn pos(&self) -> &Pos {
        &self.pos
    }
}

impl Ord for BasicCrucible {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for BasicCrucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn within_grid(pos: Pos, width: i64, height: i64) -> bool {
    pos.im >= 0 && pos.im < height && pos.re >= 0 && pos.re < height
}

type City = HashMap<Pos, u32>;

type Pos = Complex<i64>;
type Dir = Complex<i64>;

const fn lava_pool(height: i64) -> Pos {
    Pos::new(0, height - 1)
}

const fn factory(width: i64) -> Pos {
    Pos::new(width - 1, 0)
}

static NORTH: Dir = Dir::new(0, 1);
static SOUTH: Dir = Dir::new(0, -1);
static EAST: Dir = Dir::new(1, 0);
static WEST: Dir = Dir::new(-1, 0);

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use num::Complex;

use crate::{solution::Solution, util};
inventory::submit!(Solution::new(17, 1, p1));
inventory::submit!(Solution::new(17, 2, p2));
