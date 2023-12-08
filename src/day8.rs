pub fn p1(input: &str) -> String {
    let (lr_instructions, nodes) = parse_input(input);

    let steps = number_of_steps(&nodes, lr_instructions, "AAA".parse().unwrap());

    format!("Number of steps to reach ZZZ: {}", steps)
}

pub fn p2(input: &str) -> String {
    let (lr_instructions, nodes) = parse_input(input);

    let start_indices: Vec<Label> = nodes
        .keys()
        .cloned()
        .filter(|label| label.is_start())
        .collect();

    // Technically not correct for the generic case, but in my input each ghost only ever visits a single end node in their cycles.
    let periods: Vec<usize> = start_indices
        .iter()
        .map(|index| number_of_steps(&nodes, lr_instructions, index.clone()))
        .collect();

    // This only works if the start node for each ghost is part of their cycle
    // or if the distance to the end from the start node is the same as a single cycle
    // which seems to be the case in my input.
    let lcm = lowest_common_multiple(&periods);

    format!("Number of steps for all ghosts to reach **Z: {}", lcm)
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
    start_node: Label,
) -> usize {
    let mut index: &Label = &start_node;
    let mut steps: usize = 0;

    for instruction in lr_instructions.chars().cycle() {
        index = match instruction {
            'L' => &nodes[index].left,
            'R' => &nodes[index].right,
            _ => panic!("Invalid instruction"),
        };

        steps += 1;

        if index.is_end() {
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

fn lowest_common_multiple(numbers: &[usize]) -> usize {
    numbers
        .iter()
        .cloned()
        .map(prime_factors)
        .reduce(|mut lhs, rhs| {
            rhs.iter().for_each(|(key, count)| {
                lhs.insert(*key, *count.max(lhs.get(key).unwrap_or(&0)));
            });
            lhs
        })
        .unwrap()
        .iter()
        .map(|(key, count)| key.pow(*count))
        .product()
}

fn prime_factors(mut number: usize) -> HashMap<usize, u32> {
    prime_numbers(number)
        .into_iter()
        .fold(HashMap::new(), |mut acc, prime| {
            while number % prime == 0 {
                number /= prime;
                *acc.entry(prime).or_insert(0) += 1
            }
            acc
        })
}

fn prime_numbers(until: usize) -> Vec<usize> {
    let mut primes = vec![2, 3, 5, 7, 11, 13];

    for number in 15.. {
        if primes.iter().all(|prime| number % prime != 0) {
            primes.push(number)
        }

        if number > until {
            break;
        }
    }

    primes
}

use std::{collections::HashMap, str::FromStr};

use crate::solution::Solution;
inventory::submit!(Solution::new(8, 1, p1));
inventory::submit!(Solution::new(8, 2, p2));
