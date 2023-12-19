pub fn p1(input: &str) -> String {
    let mut it = input.trim().split("\n\n");
    let workflows: HashMap<String, Workflow> = it
        .next()
        .unwrap()
        .lines()
        .map(Workflow::parse)
        .map(|w| (w.name.clone(), w))
        .collect();

    let sum: u32 = it
        .next()
        .unwrap()
        .lines()
        .map(Part::parse)
        .filter(|part| is_accepted(&workflows, part))
        .map(|part| part.0.values().sum::<u32>())
        .sum();

    format!("Sum: {}", sum)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

fn is_accepted(workflows: &HashMap<String, Workflow>, part: &Part) -> bool {
    let mut workflow: String = "in".to_string();

    loop {
        workflow = workflows.get(&workflow).expect("Impossibru!").process(part);

        if workflow == "A" {
            return true;
        }

        if workflow == "R" {
            return false;
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    fallback: String,
}

impl Workflow {
    fn parse(line: &str) -> Self {
        let (name, rules, fallback) =
            sscanf::sscanf!(line.trim(), "{String}{{{String},{String:/\\w+/}}}")
                .expect("Failed to parse workflow");

        let re = Regex::new(r"(\w)([<>])(\d+):(\w+)").unwrap();

        let rules = re
            .captures_iter(&rules)
            .map(|c| {
                Rule::new(
                    c[1].chars().next().expect("Nope"),
                    c[2].chars().next().expect("Dope"),
                    c[3].parse().expect("Not a number"),
                    c[4].to_string(),
                )
            })
            .collect();

        Self {
            name,
            rules,
            fallback,
        }
    }

    fn process(&self, part: &Part) -> String {
        self.rules
            .iter()
            .find_map(|rule| rule.try_match(part))
            .unwrap_or(self.fallback.clone())
    }
}

#[derive(Debug)]
struct Rule {
    left: char,
    operator: char,
    right: u32,
    dest: String,
}

impl Rule {
    fn new(left: char, operator: char, right: u32, dest: String) -> Self {
        Self {
            left,
            operator,
            right,
            dest,
        }
    }
}

impl Rule {
    fn try_match(&self, part: &Part) -> Option<String> {
        let left_val = part.0.get(&self.left).expect("Impossibruh");

        let is_match = match self.operator {
            '<' => *left_val < self.right,
            '>' => *left_val > self.right,
            _ => panic!("Unexpected operator"),
        };

        if is_match {
            Some(self.dest.clone())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Part(HashMap<char, u32>);

impl Part {
    fn parse(line: &str) -> Self {
        let re = Regex::new(r"(\w)=(\d+)").unwrap();

        Self(
            re.captures_iter(line)
                .map(|c| {
                    (
                        c[1].chars().next().expect("Crap"),
                        c[2].parse().expect("Nope, not a digit"),
                    )
                })
                .collect(),
        )
    }
}

use std::collections::HashMap;

use regex::Regex;

use crate::solution::Solution;
inventory::submit!(Solution::new(19, 1, p1));
inventory::submit!(Solution::new(19, 2, p2));
