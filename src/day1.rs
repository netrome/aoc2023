pub fn p1(input: &str) -> String {
    let digits: Vec<Vec<u32>> = input
        .split_whitespace()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let sum: u32 = digits
        .into_iter()
        .map(|line_digits| combine_first_and_last_digit(&line_digits))
        .sum();

    format!("Sum: {}", sum)
}

pub fn p2(input: &str) -> String {
    todo!();
}

fn combine_first_and_last_digit(line_digits: &[u32]) -> u32 {
    let first = line_digits.first().expect("No first digit");
    let last = line_digits.last().expect("No last digit");

    first * 10 + last
}

use crate::solution::Solution;
inventory::submit!(Solution::new(1, 1, p1));
inventory::submit!(Solution::new(1, 2, p2));
