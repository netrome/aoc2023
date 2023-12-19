pub fn p1(input: &str) -> String {
    let mut it = input.trim().split("\n\n");
    let workflows: HashMap<String, Workflow> = it
        .next()
        .unwrap()
        .lines()
        .map(Workflow::parse)
        .map(|w| (w.name.clone(), w))
        .collect();

    let sum: u64 = it
        .next()
        .unwrap()
        .lines()
        .map(Part::parse)
        .filter(|part| is_accepted(&workflows, part))
        .map(|part| part.0.values().sum::<u64>())
        .sum();

    format!("Sum: {}", sum)
}

pub fn p2(input: &str) -> String {
    let workflows: HashMap<String, Workflow> = input
        .trim()
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(Workflow::parse)
        .map(|w| (w.name.clone(), w))
        .collect();

    let entry = "in".to_string();
    let ranges = PartRanges::start();

    let s = accepted_ranges(&workflows, ranges, entry);

    format!("Sum: {}", s)
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

fn accepted_ranges(
    workflows: &HashMap<String, Workflow>,
    ranges: PartRanges,
    entry: String,
) -> u64 {
    let workflow = workflows.get(&entry).expect("Whaaat???");

    workflow
        .compute(ranges)
        .into_iter()
        .filter(|(_, ranges)| !ranges.is_empty())
        .map(|(w, ranges)| match w.as_str() {
            "A" => ranges.size(),
            "R" => 0,
            _ => accepted_ranges(workflows, ranges, w),
        })
        .sum()
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

    fn compute(&self, ranges: PartRanges) -> Vec<(String, PartRanges)> {
        let (mut res, fallback) = self
            .rules
            .iter()
            .fold((Vec::new(), ranges), |mut acc, rule| {
                let (accepted, rejected) = acc.1.split_at(rule.left, rule.operator, rule.right);

                if !accepted.is_empty() {
                    acc.0.push((rule.dest.clone(), accepted))
                };

                (acc.0, rejected)
            });

        res.push((self.fallback.clone(), fallback));

        res
    }
}

#[derive(Debug)]
struct Rule {
    left: char,
    operator: char,
    right: u64,
    dest: String,
}

impl Rule {
    fn new(left: char, operator: char, right: u64, dest: String) -> Self {
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
struct Part(HashMap<char, u64>);

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

#[derive(Debug, Clone)]
struct PartRanges(HashMap<char, Range>);

impl PartRanges {
    fn start() -> Self {
        Self("xmas".chars().map(|c| (c, Range(1, 4000))).collect())
    }

    fn split_at(&self, c: char, op: char, val: u64) -> (PartRanges, PartRanges) {
        let mut accepted = self.clone();
        let mut rejected = self.clone();

        let (accepted_range, rejected_range) = self.0.get(&c).unwrap().split_at(op, val);

        accepted.0.insert(c, accepted_range);
        rejected.0.insert(c, rejected_range);

        (accepted, rejected)
    }

    fn is_empty(&self) -> bool {
        self.0.values().any(|range| range.is_empty())
    }

    fn size(&self) -> u64 {
        self.0.values().map(|range| range.size()).product()
    }
}

#[derive(Debug, Clone)]
struct Range(u64, u64);

impl Range {
    fn split_at(&self, op: char, val: u64) -> (Range, Range) {
        match op {
            '<' => (Self(self.0, val - 1), Self(val, self.1)),
            '>' => (Self(val + 1, self.1), Self(self.0, val)),
            _ => panic!("Unexpected op"),
        }
    }

    fn size(&self) -> u64 {
        self.1 - self.0 + 1
    }

    fn is_empty(&self) -> bool {
        self.1 <= self.0
    }
}

use std::collections::HashMap;

use regex::Regex;

use crate::solution::Solution;
inventory::submit!(Solution::new(19, 1, p1));
inventory::submit!(Solution::new(19, 2, p2));
