pub fn p1(input: &str) -> String {
    solve(input, false)
}

pub fn p2(input: &str) -> String {
    solve(input, true)
}

fn solve(input: &str, with_joker: bool) -> String {
    let mut hands: Vec<Hand> = input
        .trim()
        .lines()
        .map(|line| {
            line.parse::<Hand>()
                .expect("Failed to parse line")
                .with_joker(with_joker)
        })
        .collect();

    hands.sort();

    let total_winnings: usize = hands
        .into_iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum();

    format!("Total winnings: {}", total_winnings)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    bid: usize,
    cards: Vec<usize>,
}

impl Hand {
    fn hand_type(&self) -> usize {
        let non_jokers: Vec<usize> = self.cards.iter().cloned().filter(|c| *c != 1).collect();

        let counts: HashMap<usize, usize> =
            non_jokers.iter().fold(HashMap::new(), |mut acc, card| {
                *acc.entry(*card).or_insert(0) += 1;
                acc
            });

        let mut count_values: Vec<usize> = counts.values().cloned().collect();
        count_values.sort();

        // Insight: It is always best to turn jokers into the most common card
        let num_jokers = 5 - non_jokers.len();
        let max_count = *count_values.last().unwrap_or(&0) + num_jokers;
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

    fn with_joker(mut self, jack_is_joker: bool) -> Self {
        if jack_is_joker {
            self.cards = self
                .cards
                .into_iter()
                .map(|c| if c == 11 { 1 } else { c })
                .collect();
        }

        self
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
