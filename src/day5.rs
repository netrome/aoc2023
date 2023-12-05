pub fn p1(input: &str) -> String {
    let mut input_iter = input.trim().split("\n\n");

    let seeds: Vec<u64> = parse_seeds(input_iter.next().expect("No seed line"));

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

pub fn p2(input: &str) -> String {
    let mut input_iter = input.trim().split("\n\n");

    let seeds: Vec<u64> = parse_seeds(input_iter.next().expect("No seed line"));
    let seed_ranges: Vec<SourceRange> = seeds
        .windows(2)
        .map(|window| SourceRange {
            start: *window.first().unwrap(),
            length: *window.last().unwrap(),
        })
        .collect();

    let maps: Vec<Map> = input_iter.map(|input| input.parse().unwrap()).collect();

    let init_chained_map: ChainedMap = maps.first().unwrap().clone().into();

    let chained_map = maps
        .into_iter()
        .skip(1)
        .fold(init_chained_map, |chained_map, map| chained_map.chain(map));

    let mapped_ranges = chained_map.map_ranges(seed_ranges);

    let lowest_location = mapped_ranges
        .into_iter()
        .map(|range| range.start)
        .min()
        .expect("No range");

    format!("Lowest location: {}", lowest_location)
}

fn parse_seeds(s: &str) -> Vec<u64> {
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

    fn map(&self, source: u64) -> u64 {
        self.maps.iter().fold(source, |acc, map| map.map(acc))
    }

    fn map_ranges(&self, source_ranges: Vec<SourceRange>) -> Vec<SourceRange> {
        self.maps.iter().fold(source_ranges, |acc, map| {
            acc.into_iter()
                .flat_map(|source_range| map.map_range(source_range))
                .collect()
        })
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
    fn map(&self, source: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| range.try_map(source))
            .unwrap_or(source)
    }

    fn map_range(&self, source_range: SourceRange) -> Vec<SourceRange> {
        println!("Map: {:?}", &self);
        println!("Source range: {:?}", &source_range);

        let unmapped_below = self
            .ranges
            .first()
            .expect("No first range")
            .unmapped_range_below(source_range);

        let unmapped_above = self
            .ranges
            .last()
            .expect("No last range")
            .unmapped_range_above(source_range);

        let mapped_ranges = unmapped_below
            .into_iter()
            .chain(
                self.ranges
                    .iter()
                    .filter_map(|range| range.mapped_range(source_range)),
            )
            .chain(unmapped_above.into_iter())
            .collect();

        println!("Mapped ranges: {:?}", mapped_ranges);
        println!("---");

        mapped_ranges
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

        let mut ranges: Vec<Range> = lines
            .map(|line| line.parse().expect("Failed to parse range"))
            .collect();

        ranges.sort_by_key(|range| range.source_start);

        Ok(Self {
            source,
            destination,
            ranges,
        })
    }
}

#[derive(Debug, Clone)]
struct Range {
    source_start: u64,
    destination_start: u64,
    length: u64,
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_start, source_start, length) =
            sscanf::sscanf!(s.trim(), "{u64} {u64} {u64}").expect("Failed to read range");

        Ok(Self {
            source_start,
            destination_start,
            length,
        })
    }
}

impl Range {
    fn try_map(&self, source: u64) -> Option<u64> {
        let offset = source.checked_sub(self.source_start)?;

        if offset < self.length {
            Some(self.destination_start + offset)
        } else {
            None
        }
    }

    fn unmapped_range_below(&self, source_range: SourceRange) -> Option<SourceRange> {
        let diff = self.source_start.checked_sub(source_range.start)?;
        let length = source_range.length.min(diff);

        Some(SourceRange {
            start: source_range.start,
            length,
        })
    }

    fn unmapped_range_above(&self, source_range: SourceRange) -> Option<SourceRange> {
        let diff = (source_range.start + source_range.length)
            .checked_sub(self.source_start + self.length)?;

        let start = (self.source_start + self.length).max(source_range.start);
        let length = source_range.length.min(diff);

        Some(SourceRange { start, length })
    }

    fn mapped_range(&self, source_range: SourceRange) -> Option<SourceRange> {
        let translation = self.destination_start.abs_diff(self.source_start);

        let source_intersection_start = source_range.start.max(self.source_start);
        let end = (source_range.start + source_range.length).min(self.source_start + self.length);
        let length = end.checked_sub(source_intersection_start)?;

        let start = if self.destination_start > self.source_start {
            source_intersection_start + translation
        } else {
            source_intersection_start - translation
        };

        Some(SourceRange { start, length })
    }
}

#[derive(Debug, Clone, Copy)]
struct SourceRange {
    start: u64,
    length: u64,
}

use std::str::FromStr;

use itertools::Itertools;

use crate::solution::Solution;
inventory::submit!(Solution::new(5, 1, p1));
inventory::submit!(Solution::new(5, 2, p2));
