pub fn p1(input: &str) -> String {
    let (lr_instructions, nodes) = parse_input(input);

    let steps = number_of_steps(&nodes, &lr_instructions, &["AAA".parse().unwrap()]);

    format!("Number of steps to reach ZZZ: {}", steps)
}

pub fn p2(input: &str) -> String {
    let (lr_instructions, nodes) = parse_input(input);

    let start_indices: Vec<Label> = nodes
        .keys()
        .cloned()
        .filter(|label| label.is_start())
        .collect();

    let steps = number_of_steps(&nodes, &lr_instructions, &start_indices);

    format!("Number of steps for all ghosts to reach **Z: {}", steps)
}

fn parse_input(input: &str) -> (&str, HashMap<Label, Node>) {
    let mut input_iter = input.trim().lines();
    let lr_instructions = input_iter.next().unwrap().trim();

    // Skip blank line
    input_iter.next();

    let nodes: HashMap<Label, Node> = input_iter
        .map(|line| line.parse::<Node>().expect("Failed to parse node"))
        .map(|node| (node.label.clone(), node))
        .collect();

    (lr_instructions, nodes)
}

fn number_of_steps(
    nodes: &HashMap<Label, Node>,
    lr_instructions: &str,
    start_nodes: &[Label],
) -> usize {
    let mut indices: Vec<&Label> = start_nodes.iter().collect();
    let mut steps: usize = 0;

    for instruction in lr_instructions.chars().cycle() {
        for index in indices.iter_mut() {
            *index = match instruction {
                'L' => &nodes[index].left,
                'R' => &nodes[index].right,
                _ => panic!("Invalid instruction"),
            };
        }
        steps += 1;

        if indices.iter().all(|index| index.is_end()) {
            break;
        }
    }

    steps
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

impl Label {
    fn is_start(&self) -> bool {
        self.0[2] == 'A'
    }

    fn is_end(&self) -> bool {
        self.0[2] == 'Z'
    }
}

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
