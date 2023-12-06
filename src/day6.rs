pub fn p1(input: &str) -> String {
    let mut it = input.trim().lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i32>>()
    });

    let n: usize = it
        .next()
        .unwrap()
        .into_iter()
        .zip(it.next().unwrap())
        .map(number_of_ways_to_win)
        .product();

    format!("Number of ways to win: {:?}", n)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

fn number_of_ways_to_win((time, record): (i32, i32)) -> usize {
    (0..time).filter(|x| x * (time - x) > record).count()
}

use crate::solution::Solution;
inventory::submit!(Solution::new(6, 1, p1));
inventory::submit!(Solution::new(6, 2, p2));
