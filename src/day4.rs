pub fn p1(input: &str) -> String {
    let total_points: u32 = input
        .trim()
        .lines()
        .map(|line| line.parse::<Card>().unwrap().value())
        .sum();

    format!("Total points: {}", total_points)
}

pub fn p2(input: &str) -> String {
    let mut cards: Vec<Card> = input
        .trim()
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect();

    for idx in 0..cards.len() {
        let card = cards.get(idx).unwrap().clone();

        for idx2 in (idx + 1)..(idx + card.number_of_winning_numbers() + 1) {
            cards
                .get_mut(idx2)
                .expect("Could not find card at index")
                .copies += card.copies;
        }
    }

    let number_of_cards: usize = cards.into_iter().map(|card| card.copies).sum();

    format!("Total scratchcards: {}", number_of_cards)
}

#[derive(Debug, Clone)]
struct Card {
    _id: usize,
    copies: usize,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

impl Card {
    fn number_of_winning_numbers(&self) -> usize {
        self.numbers_you_have
            .iter()
            .cloned()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    fn value(&self) -> u32 {
        match self.number_of_winning_numbers() {
            0 => 0,
            len => 2_u32.pow(len as u32 - 1),
        }
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, id, list_1, list_2) =
            sscanf::sscanf!(s.trim(), "Card{String}{usize}: {String} | {String}")
                .expect(&format!("Failed to scan string: {}", s));

        let winning_numbers = list_1
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let numbers_you_have = list_2
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let copies = 1;

        Ok(Self {
            _id: id,
            copies,
            winning_numbers,
            numbers_you_have,
        })
    }
}

use std::{
    collections::{BTreeSet, HashSet},
    str::FromStr,
};

use crate::solution::Solution;
inventory::submit!(Solution::new(4, 1, p1));
inventory::submit!(Solution::new(4, 2, p2));
