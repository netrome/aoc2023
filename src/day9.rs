pub fn p1(input: &str) -> String {
    let sum: i64 = input
        .lines()
        .map(read_line)
        .map(|c| extrapolate(&c))
        .map(|seq| *seq.last().unwrap())
        .sum();

    format!("Sum: {}", sum)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

fn read_line(line: &str) -> Vec<i64> {
    let values = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let diffs = all_diffs(values);
    let current = current_value_with_all_diffs(&diffs);
    current
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

use crate::solution::Solution;
inventory::submit!(Solution::new(9, 1, p1));
inventory::submit!(Solution::new(9, 2, p2));
