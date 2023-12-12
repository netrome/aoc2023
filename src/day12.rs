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

pub fn p2(input: &str) -> String {
    let mut rows: Vec<Row> = input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    rows.iter_mut().for_each(|row| row.unfold());

    let sum: usize = rows
        .into_iter()
        .map(|row| dbg!(row.number_of_possible_arrangements()))
        .sum();

    format!("Sum: {}", sum)
}

struct Row {
    condition_records: Vec<char>,
    groups: Vec<usize>,
}

impl Row {
    fn number_of_possible_arrangements(&self) -> usize {
        number_of_matches(&self.condition_records, &self.groups, 0)
    }

    fn unfold(&mut self) {
        let mut condition_records = self.condition_records.clone();
        let mut groups = self.groups.clone();
        for _ in 0..4 {
            condition_records.push('?');
            condition_records.extend_from_slice(&self.condition_records);
            groups.extend_from_slice(&self.groups);
        }

        self.condition_records = condition_records;
        self.groups = groups;
    }
}

fn is_match2(condition_records: &[char], at: usize, len: usize) -> bool {
    let edges_could_be_operational = *condition_records
        .get(at.checked_sub(1).unwrap_or(usize::MAX))
        .unwrap_or(&'.')
        != '#'
        && *condition_records.get(at + len).unwrap_or(&'.') != '#';

    let springs_could_be_damaged = condition_records
        .get(at..at + len)
        .map(|slice| slice.len() == len && slice.iter().all(|c| *c != '.'))
        .unwrap_or(false);

    edges_could_be_operational && springs_could_be_damaged
}

fn number_of_matches(condition_records: &[char], groups: &[usize], start: usize) -> usize {
    if let Some((group, remaining_groups)) = groups.split_first() {
        let mut ans = 0;
        let remaining_len: usize = remaining_groups.iter().sum();
        for at in start..(condition_records.len() - group - remaining_len + 1) {
            if is_match2(condition_records, at, *group) {
                ans += number_of_matches(condition_records, remaining_groups, at + *group + 1);
            }

            if condition_records[at] == '#' {
                break;
            }
        }
        ans
    } else {
        condition_records
            .get(start..)
            .map(|slice| !slice.into_iter().any(|c| *c == '#'))
            .unwrap_or(true) as usize
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching_works() {
        assert!(match_str("???.###", 4, 3));
        assert!(!match_str("???.###", 3, 3));
        assert!(!match_str("???.###", 1, 3));
        assert!(match_str("???.###", 0, 3));
        assert!(match_str("???.###", 0, 1));
        assert!(match_str("???.###", 1, 1));
        assert!(match_str("???.###", 2, 1));
        assert!(!match_str("???.###", 3, 1));
        assert!(!match_str("???.###", 4, 1));
        assert!(!match_str("???.###", 5, 1));
        assert!(!match_str("???.###", 6, 1));
    }

    #[test]
    fn numbers_are_right() {
        assert_eq!(num_matches_str("???.### 1,1,3"), 1);
        assert_eq!(num_matches_str("?###???????? 3,2,1"), 10);
        assert_eq!(num_matches_str("??#??????#???.? 4,3"), 9);
        assert_eq!(num_matches_str(".??#.?????..????#?. 1,1,5"), 12);
        assert_eq!(
            num_matches_str(
                "???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"
            ),
            1
        );
    }

    fn num_matches_str(row: &str) -> usize {
        row.parse::<Row>()
            .unwrap()
            .number_of_possible_arrangements()
    }

    fn match_str(rec: &str, at: usize, len: usize) -> bool {
        is_match2(&rec.chars().collect::<Vec<_>>(), at, len)
    }
}
