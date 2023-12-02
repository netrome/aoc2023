pub fn p1(input: &str) -> String {
    todo!();
}

pub fn p2(_input: &str) -> String {
    todo!();
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

#[derive(Debug)]
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
