pub fn p1(input: &str) -> String {
    solve(input, parse_p1)
}

pub fn p2(input: &str) -> String {
    assert_eq!(parse_p2("R 6 (#70c710)"), (EAST, 461937));
    solve(input, parse_p2)
}

pub fn solve(input: &str, parse: impl FnMut(&str) -> (Dir, i64)) -> String {
    let (_, area, len) = input.trim().lines().map(parse).fold(
        (Pos::new(0, 0), 0, 0),
        |(last_corner, mut shoelace_area, trench_len), (dir, dist)| {
            let next_corner = last_corner + (dir * dist);
            shoelace_area += last_corner.re * next_corner.im - last_corner.im * next_corner.re;

            (next_corner, shoelace_area, trench_len + dist)
        },
    );

    format!("Area: {}", area.abs() / 2 + len / 2 + 1)
}

fn parse_p1(line: &str) -> (Dir, i64) {
    let mut it = line.split_whitespace();
    let dirchar = it.next().unwrap();
    let dist = it.next().unwrap().parse().unwrap();

    let dir = match dirchar {
        "U" => NORTH,
        "D" => SOUTH,
        "L" => WEST,
        "R" => EAST,
        _ => panic!("Unexpected char"),
    };
    (dir, dist)
}

fn parse_p2(line: &str) -> (Dir, i64) {
    let hex = line.split_whitespace().last().unwrap();
    let dist_str = hex.get(2..hex.len() - 2).unwrap();

    let dir = match hex.chars().rev().nth(1).unwrap() {
        '0' => EAST,
        '1' => SOUTH,
        '2' => WEST,
        '3' => NORTH,
        _ => panic!("Unexpected"),
    };

    let dist = i64::from_str_radix(dist_str, 16).unwrap();

    (dir, dist)
}

type Pos = Complex<i64>;
type Dir = Complex<i64>;

static NORTH: Dir = Dir::new(0, 1);
static SOUTH: Dir = Dir::new(0, -1);
static EAST: Dir = Dir::new(1, 0);
static WEST: Dir = Dir::new(-1, 0);

use num::Complex;

use crate::solution::Solution;
inventory::submit!(Solution::new(18, 1, p1));
inventory::submit!(Solution::new(18, 2, p2));
