pub fn p1(input: &str) -> String {
    let sum: i64 = input
        .lines()
        .map(read_line)
        .map(|seq| extrapolate(&current_value_with_all_diffs(&all_diffs(seq))))
        .map(|seq| *seq.last().unwrap())
        .sum();

    format!("Sum: {}", sum)
}

pub fn p2(input: &str) -> String {
    let sum: i64 = input
        .lines()
        .map(read_line)
        .map(|seq| extrapolate_backwards(&first_value_with_backward_diffs(&all_diffs(seq))))
        .map(|seq| *seq.last().unwrap())
        .sum();

    format!("Sum: {}", sum)
}

fn read_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn diff(seq: &[i64]) -> Vec<i64> {
    seq.windows(2).map(|w| w[1] - w[0]).collect()
}

fn all_diffs(seq: Vec<i64>) -> Vec<Vec<i64>> {
    let mut current = diff(&seq);
    let mut diffs = vec![seq];

    while current.iter().any(|x| *x != 0) {
        let next_diff = diff(&current);
        diffs.push(current);
        current = next_diff;
    }

    diffs
}

fn current_value_with_all_diffs(diffs: &[Vec<i64>]) -> Vec<i64> {
    diffs
        .iter()
        .rev()
        .map(|seq| *seq.last().expect("Empty sequence"))
        .collect()
}

fn first_value_with_backward_diffs(diffs: &[Vec<i64>]) -> Vec<i64> {
    diffs
        .iter()
        .rev()
        .map(|seq| *seq.first().expect("Empty sequence"))
        .collect()
}

fn extrapolate(current: &[i64]) -> Vec<i64> {
    let mut cumsum = 0;

    current
        .iter()
        .map(|val| {
            cumsum += val;
            cumsum
        })
        .collect()
}

fn extrapolate_backwards(first: &[i64]) -> Vec<i64> {
    let mut cumsum = 0;

    first
        .iter()
        .map(|val| {
            cumsum -= val;
            cumsum *= -1;
            cumsum
        })
        .collect()
}

use crate::solution::Solution;
inventory::submit!(Solution::new(9, 1, p1));
inventory::submit!(Solution::new(9, 2, p2));
