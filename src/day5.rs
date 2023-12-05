pub fn p1(input: &str) -> String {
    let mut input_iter = input.trim().split("\n\n");

    let seeds: Vec<u32> = parse_seeds(input_iter.next().expect("No seed line"));

    let maps: Vec<Map> = input_iter.map(|input| input.parse().unwrap()).collect();

    let init_chained_map: ChainedMap = maps.first().unwrap().clone().into();

    let chained_map = maps
        .into_iter()
        .skip(1)
        .fold(init_chained_map, |chained_map, map| chained_map.chain(map));

    let lowest_location = seeds
        .into_iter()
        .map(|seed| chained_map.map(seed))
        .min()
        .expect("No min value");

    format!("Lowest location: {}", lowest_location)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

fn parse_seeds(s: &str) -> Vec<u32> {
    s.split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|digit_str| digit_str.parse().expect("Not a digit"))
        .collect()
}

struct ChainedMap {
    source: String,
    destination: String,
    maps: Vec<Map>,
}

impl ChainedMap {
    fn chain(mut self, map: Map) -> Self {
        assert_eq!(self.destination, map.source);

        self.destination = map.destination.clone();
        self.maps.push(map);

        self
    }

    fn map(&self, source: u32) -> u32 {
        self.maps.iter().fold(source, |acc, map| map.map(acc))
    }
}

impl From<Map> for ChainedMap {
    fn from(value: Map) -> Self {
        Self {
            source: value.source.clone(),
            destination: value.destination.clone(),
            maps: vec![value],
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    source: String,
    destination: String,
    ranges: Vec<Range>,
}

impl Map {
    fn map(&self, source: u32) -> u32 {
        self.ranges
            .iter()
            .find_map(|range| range.try_map(source))
            .unwrap_or(source)
    }
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();
        let (source, destination) = sscanf::sscanf!(
            lines.next().expect("No map header").trim(),
            "{String}-to-{String} map:"
        )
        .expect("Failed to parse map header");

        let ranges = lines
            .map(|line| line.parse().expect("Failed to parse range"))
            .collect();

        Ok(Self {
            source,
            destination,
            ranges,
        })
    }
}

#[derive(Debug, Clone)]
struct Range {
    source_start: u32,
    destination_start: u32,
    length: u32,
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_start, source_start, length) =
            sscanf::sscanf!(s.trim(), "{u32} {u32} {u32}").expect("Failed to read range");

        Ok(Self {
            source_start,
            destination_start,
            length,
        })
    }
}

impl Range {
    fn try_map(&self, source: u32) -> Option<u32> {
        let offset = source.checked_sub(self.source_start)?;

        if offset < self.length {
            Some(self.destination_start + offset)
        } else {
            None
        }
    }
}

use std::str::FromStr;

use crate::solution::Solution;
inventory::submit!(Solution::new(5, 1, p1));
inventory::submit!(Solution::new(5, 2, p2));
