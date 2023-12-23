pub fn p1(input: &str) -> String {
    let map: Map = util::char_grid_iter(input)
        .map(|(x, y, c)| (Pos::new(x as i64, y as i64), c))
        .collect();

    let (w, h) = util::grid_dimensions(input);
    let w = w as i64;
    let h = h as i64;

    let before = Pos::new(1, h);
    let start = Pos::new(1, h - 1);
    let end = Pos::new(w - 2, 0);

    let hikes = find_hikes(&map, vec![before, start], end);
    let longest = hikes.iter().map(|hike| hike.len() - 2).max().unwrap();

    format!("Longest hike: {}", longest)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

fn find_hikes(map: &Map, history: Vec<Pos>, dest: Pos) -> Vec<Vec<Pos>> {
    let from = history.last().expect("Empty history, should not happen");
    let before = history
        .iter()
        .rev()
        .nth(1)
        .expect("History should always have minimum two elements");

    possible_steps(map, *from)
        .into_iter()
        .filter(|step| step != before)
        .flat_map(|step| {
            let mut hist = history.clone();
            hist.push(step);
            if step == dest {
                vec![hist]
            } else {
                find_hikes(map, hist, dest)
            }
        })
        .collect()
}

fn possible_steps(map: &Map, from: Pos) -> Vec<Pos> {
    let candidates = match map.get(&from).expect("Outside of map") {
        '.' => vec![from + NORTH, from + SOUTH, from + EAST, from + WEST],
        '>' => vec![from + EAST],
        '<' => vec![from + WEST],
        '^' => vec![from + NORTH],
        'v' => vec![from + SOUTH],
        _ => panic!("Invalid terrain"),
    };

    candidates
        .into_iter()
        .filter(|pos| *map.get(pos).unwrap_or(&'#') != '#')
        .collect()
}

type Map = HashMap<Pos, char>;

type Pos = Complex<i64>;
type Dir = Complex<i64>;

static NORTH: Dir = Dir::new(0, 1);
static SOUTH: Dir = Dir::new(0, -1);
static EAST: Dir = Dir::new(1, 0);
static WEST: Dir = Dir::new(-1, 0);

use std::collections::HashMap;

use num::Complex;

use crate::{solution::Solution, util};
inventory::submit!(Solution::new(23, 1, p1));
inventory::submit!(Solution::new(23, 2, p2));
