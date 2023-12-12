pub fn p1(input: &str) -> String {
    let rows: Vec<Row> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let sum: usize = rows
        .into_iter()
        .map(|row| row.number_of_possible_arrangements())
        .sum();

    format!("Sum: {}", sum)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

struct Row {
    condition_records: Vec<char>,
    groups: Vec<usize>,
}

impl Row {
    fn number_of_possible_arrangements(&self) -> usize {
        let arrangements =
            matching_arrangements(&self.condition_records, &self.groups, Vec::new(), 0);

        arrangements.len()
    }
}

fn matching_arrangements(
    condition_records: &[char],
    groups: &[usize],
    acc: Vec<usize>,
    index: usize,
) -> Vec<Vec<usize>> {
    if let Some((group, remaining_groups)) = groups.split_first() {
        let mut res = if is_match(*group, condition_records) {
            let mut next_acc = acc.clone();
            next_acc.push(index);

            if let Some(next_records) = condition_records.get((group + 1)..) {
                matching_arrangements(next_records, remaining_groups, next_acc, index + group + 1)
            } else {
                if remaining_groups.is_empty() {
                    vec![next_acc]
                } else {
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        };

        let without_match = if let Some((rec, next_records)) = condition_records.split_first() {
            if *rec != '#' {
                matching_arrangements(next_records, groups, acc, index + 1)
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };

        res.extend_from_slice(&without_match);

        res
    } else {
        if condition_records.iter().any(|c| *c == '#') {
            Vec::new()
        } else {
            vec![acc]
        }
    }
}

fn is_match(group: usize, records: &[char]) -> bool {
    records.len() >= group
        && records.iter().take(group).all(|c| *c != '.')
        && *records.get(group).unwrap_or(&'.') != '#'
}

impl FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let condition_records = it.next().unwrap().chars().collect();
        let groups = it
            .next()
            .unwrap()
            .split(',')
            .map(|d| d.parse().unwrap())
            .collect();

        Ok(Self {
            condition_records,
            groups,
        })
    }
}

use std::str::FromStr;

use crate::solution::Solution;
inventory::submit!(Solution::new(12, 1, p1));
inventory::submit!(Solution::new(12, 2, p2));
