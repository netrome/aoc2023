pub fn p1(input: &str) -> String {
    let map: Map = util::char_grid_iter(input)
        .map(|(x, y, c)| (Pos::new(x as i32, y as i32), c))
        .collect();

    let (w, h) = util::grid_dimensions(input);
    let w = w as i32;
    let h = h as i32;

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
            (Pos::new(x as i32, y as i32), c2)
        })
        .collect();

    let (w, h) = util::grid_dimensions(input);
    let w = w as i32;
    let h = h as i32;

    let start = Pos::new(1, h - 1);
    let end = Pos::new(w - 2, 0);

    let mut intersections: HashSet<Pos> = map
        .iter()
        .filter(|(_, c)| **c == '.')
        .map(|(pos, _)| *pos)
        .filter(|pos| possible_steps(&map, *pos).len() > 2)
        .collect();

    intersections.insert(start);
    intersections.insert(end);

    let graph: Graph = intersections
        .iter()
        .map(|pos| {
            (
                *pos,
                find_neighbor_intersections(&map, &intersections, *pos),
            )
        })
        .collect();

    let longest = hike_lengths(&graph, HashSet::new(), 0, start, end)
        .into_iter()
        .max()
        .unwrap();

    format!("Longest hike: {}", longest)
}

fn hike_lengths(
    graph: &Graph,
    mut ignore: HashSet<Pos>,
    at: usize,
    from: Pos,
    to: Pos,
) -> Vec<usize> {
    if from == to {
        vec![at]
    } else {
        ignore.insert(from);
        graph
            .get(&from)
            .unwrap()
            .iter()
            .filter(|(pos, _)| !ignore.contains(pos))
            .flat_map(|(pos, dist)| hike_lengths(graph, ignore.clone(), at + dist, *pos, to))
            .collect()
    }
}

fn find_neighbor_intersections(
    map: &Map,
    intersections: &HashSet<Pos>,
    i1: Pos,
) -> Vec<(Pos, usize)> {
    let mut seen = HashSet::new();
    seen.insert(i1);

    let mut cursors: Vec<_> = possible_steps(&map, i1)
        .into_iter()
        .map(|pos| (pos, 1))
        .collect();
    let mut res = Vec::new();

    while let Some((pos, dist)) = cursors.pop() {
        if intersections.contains(&pos) {
            res.push((pos, dist))
        } else {
            seen.insert(pos);

            let next_step = possible_steps(map, pos)
                .into_iter()
                .filter(|step| !seen.contains(step))
                .next()
                .unwrap(); // Should always be exactly one here unless we have a self-loop or an unknown intersection

            cursors.push((next_step, dist + 1))
        }
    }

    res
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

type Graph = HashMap<Pos, Vec<(Pos, usize)>>;
type Map = HashMap<Pos, char>;

type Pos = Complex<i32>;
type Dir = Complex<i32>;

static NORTH: Dir = Dir::new(0, 1);
static SOUTH: Dir = Dir::new(0, -1);
static EAST: Dir = Dir::new(1, 0);
static WEST: Dir = Dir::new(-1, 0);

use std::collections::{HashMap, HashSet};

use num::Complex;

use crate::{solution::Solution, util};
inventory::submit!(Solution::new(23, 1, p1));
inventory::submit!(Solution::new(23, 2, p2));
