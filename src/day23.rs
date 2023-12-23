pub fn p1(input: &str) -> String {
    let map: Map = util::char_grid_iter(input)
        .map(|(x, y, c)| (Pos::new(x as i64, y as i64), c))
        .collect();

    let (w, h) = util::grid_dimensions(input);
    let w = w as i64;
    let h = h as i64;

    let start = Pos::new(1, h - 1);
    let end = Pos::new(w - 2, 0);

    let longest = find_hikes(&map, [start].into_iter().collect(), start, end)
        .into_iter()
        .max()
        .unwrap();

    format!("Longest hike: {}", longest)
}

pub fn p2(input: &str) -> String {
    let map: Map = util::char_grid_iter(input)
        .map(|(x, y, c)| {
            let c2 = match c {
                '#' => '#',
                _ => '.',
            };
            (Pos::new(x as i64, y as i64), c2)
        })
        .collect();

    let (w, h) = util::grid_dimensions(input);
    let w = w as i64;
    let h = h as i64;

    let start = Pos::new(1, h - 1);
    let end = Pos::new(w - 2, 0);

    let longest = find_hikes(&map, [start].into_iter().collect(), start, end)
        .into_iter()
        .max()
        .unwrap();

    format!("Longest hike: {}", longest)
}

fn find_hikes(map: &Map, mut seen: HashSet<Pos>, from: Pos, dest: Pos) -> Vec<usize> {
    let next_steps: Vec<Pos> = possible_steps(map, from)
        .into_iter()
        .filter(|step| !seen.contains(step))
        .collect();

    match next_steps.as_slice() {
        [] => vec![],
        [step] => {
            if *step == dest {
                vec![seen.len()]
            } else {
                seen.insert(*step);
                find_hikes(map, seen, *step, dest)
            }
        }
        steps => steps
            .into_iter()
            .flat_map(|step| {
                let mut s2 = seen.clone();
                s2.insert(*step);
                find_hikes(map, s2, *step, dest)
            })
            .collect(),
    }
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

use std::collections::{HashMap, HashSet};

use num::Complex;

use crate::{solution::Solution, util};
inventory::submit!(Solution::new(23, 1, p1));
inventory::submit!(Solution::new(23, 2, p2));
