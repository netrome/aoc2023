pub fn p1(input: &str) -> String {
    let mut input_iter = input.trim().lines();
    let lr_instructions = input_iter.next().unwrap().trim();

    // Skip blank line
    input_iter.next();

    let nodes: HashMap<Label, Node> = input_iter
        .map(|line| line.parse::<Node>().expect("Failed to parse node"))
        .map(|node| (node.label.clone(), node))
        .collect();

    let start: Label = "AAA".parse().unwrap();
    let end: Label = "ZZZ".parse().unwrap();
    let mut index = &start;
    let mut steps: usize = 0;

    for instruction in lr_instructions.chars().cycle() {
        index = match instruction {
            'L' => &nodes[index].left,
            'R' => &nodes[index].right,
            _ => panic!("Invalid instruction"),
        };

        steps += 1;

        if *index == end {
            break;
        }
    }

    format!("Number of steps to reach zzz: {}", steps)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

#[derive(Debug)]
struct Node {
    label: Label,
    left: Label,
    right: Label,
}

impl FromStr for Node {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (label, left, right) = sscanf::sscanf!(s.trim(), "{Label} = ({Label}, {Label})")?;

        Ok(Self { label, left, right })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Label([char; 3]);

impl FromStr for Label {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.trim().chars().collect();

        Ok(Self(chars.try_into().unwrap()))
    }
}

impl sscanf::RegexRepresentation for Label {
    const REGEX: &'static str = r"\w\w\w";
}

use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(8, 1, p1));
inventory::submit!(Solution::new(8, 2, p2));
