pub fn p1(input: &str) -> String {
    let mut it = input.trim().lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect::<Vec<i64>>()
    });

    let n: usize = it
        .next()
        .unwrap()
        .into_iter()
        .zip(it.next().unwrap())
        .map(|(time, record)| (0..time).filter(|x| x * (time - x) > record).count())
        .product();

    format!("Number of ways to win: {}", n)
}

pub fn p2(input: &str) -> String {
    let input = input.trim().replace(' ', "").replace(':', " ");
    p1(&input)
}

use crate::solution::Solution;
inventory::submit!(Solution::new(6, 1, p1));
inventory::submit!(Solution::new(6, 2, p2));
