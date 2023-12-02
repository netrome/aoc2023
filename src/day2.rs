pub fn p1(input: &str) -> String {
    let game_sum: usize = parse_input(input)
        .into_iter()
        .filter(is_p1_possible)
        .map(|game| game.id)
        .sum();

    format!("Game ID sum: {}", game_sum)
}

pub fn p2(input: &str) -> String {
    let power_set_sum: u32 = parse_input(input)
        .into_iter()
        .map(|game| game.fewest_number_of_cubes().power_set())
        .sum();

    format!("Power set sum: {}", power_set_sum)
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            line.parse::<Game>()
                .expect(&format!("Failed to parse game from line: {}", line))
        })
        .collect()
}

fn is_p1_possible(game: &Game) -> bool {
    game.handfuls.iter().all(handful_is_p1_possible)
}

fn handful_is_p1_possible(handful: &Handful) -> bool {
    *handful.0.get(&Color::Red).unwrap_or(&0) <= 12
        && *handful.0.get(&Color::Green).unwrap_or(&0) <= 13
        && *handful.0.get(&Color::Blue).unwrap_or(&0) <= 14
}

#[derive(Debug)]
struct Game {
    id: usize,
    handfuls: Vec<Handful>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, handfuls_unparsed) =
            sscanf::sscanf!(s, "Game {usize}: {String}").expect("Failed to scan game");

        let handfuls = handfuls_unparsed
            .split(";")
            .into_iter()
            .map(Handful::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Self { id, handfuls })
    }
}

impl Game {
    fn fewest_number_of_cubes(&self) -> Handful {
        self.handfuls
            .clone()
            .into_iter()
            .reduce(|acc, element| acc.union(&element))
            .unwrap()
    }
}

#[derive(Debug, Clone)]
struct Handful(HashMap<Color, u32>);

impl FromStr for Handful {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let handful = s
            .trim()
            .split(",")
            .into_iter()
            .map(|entry| {
                sscanf::sscanf!(entry.trim(), "{u32} {Color}").expect("Failed to scan cubes")
            })
            .map(|(amount, color)| (color, amount))
            .collect();

        Ok(Handful(handful))
    }
}

impl Handful {
    fn union(&self, other: &Self) -> Self {
        Self(
            [Color::Red, Color::Green, Color::Blue]
                .into_iter()
                .map(|color| {
                    (
                        color,
                        *self
                            .0
                            .get(&color)
                            .unwrap_or(&0)
                            .max(other.0.get(&color).unwrap_or(&0)),
                    )
                })
                .collect(),
        )
    }

    fn power_set(&self) -> u32 {
        self.0.values().product()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, sscanf::FromScanf)]
enum Color {
    #[sscanf(format = "red")]
    Red,
    #[sscanf(format = "green")]
    Green,
    #[sscanf(format = "blue")]
    Blue,
}

use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(2, 1, p1));
inventory::submit!(Solution::new(2, 2, p2));

#[cfg(test)]
mod tests {
    #[test]
    fn parsing_works() {
        let game: Game = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            .parse()
            .unwrap();

        assert_eq!(*game.handfuls[0].0.get(&Color::Red).unwrap(), 3);
    }

    use super::*;
}
