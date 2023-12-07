pub fn p1(input: &str) -> String {
    let mut hands: Vec<Hand> = input
        .trim()
        .lines()
        .map(|line| line.parse().expect("Failed to parse line"))
        .collect();

    hands.sort();

    let total_winnings: usize = hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum();

    format!("Total winnings: {}", total_winnings)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    bid: usize,
    cards: Vec<usize>,
}

impl Hand {
    fn hand_type(&self) -> usize {
        let counts: HashMap<usize, usize> =
            self.cards.iter().fold(HashMap::new(), |mut acc, card| {
                *acc.entry(*card).or_insert(0) += 1;
                acc
            });

        let mut count_values: Vec<usize> = counts.values().cloned().collect();
        count_values.sort();

        let max_count = *count_values.last().unwrap();
        let maybe_second_max_count = count_values.iter().rev().nth(1);

        match (max_count, maybe_second_max_count) {
            (5, None) => 7,
            (4, Some(1)) => 6,
            (3, Some(2)) => 5,
            (3, Some(1)) => 4,
            (2, Some(2)) => 3,
            (2, Some(1)) => 2,
            (1, Some(1)) => 1,
            _ => panic!("Unanticipated edge case"),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = sscanf::sscanf!(s.trim(), "{String} {usize}").expect("Malformed string");

        let cards = cards.chars().map(parse_card).collect();

        Ok(Self { bid, cards })
    }
}

fn parse_card(c: char) -> usize {
    static VALS: Lazy<HashMap<char, usize>> = Lazy::new(|| {
        "23456789TJQKA"
            .chars()
            .enumerate()
            .map(|(idx, char)| (char, idx + 2))
            .collect()
    });

    *VALS.get(&c).expect("Invalid char")
}

use std::{cmp::Ordering, collections::HashMap, str::FromStr};

use once_cell::sync::Lazy;

use crate::solution::Solution;
inventory::submit!(Solution::new(7, 1, p1));
inventory::submit!(Solution::new(7, 2, p2));
