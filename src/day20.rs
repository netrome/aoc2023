pub fn p1(input: &str) -> String {
    let mut modules = parse_input(input);

    let (mut low, mut high) = (0, 0);

    for _ in 0..1000 {
        let (l2, h2) = push_button(&mut modules);
        low += l2;
        high += h2;
    }

    format!("Product: {}", low * high)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut builders = Vec::new();
    let mut inputs = HashMap::new();

    let re = Regex::new(r"([%&]?)(\w+) -> (.*)").unwrap();

    for line in input.trim().lines() {
        let c = re.captures(line).expect("Regex failure");

        let prefix = c[1].chars().next().unwrap_or('_');
        let name = c[2].to_string();
        let dest: Vec<String> = c[3]
            .split(',')
            .into_iter()
            .map(|s| s.trim().to_string())
            .collect();

        for d in dest.iter() {
            inputs
                .entry(d.to_string())
                .or_insert_with(Vec::new)
                .push(name.clone());
        }

        builders.push(ModuleBuilder { name, dest, prefix });
    }

    builders
        .into_iter()
        .map(|builder| {
            let default = Vec::new();
            let inputs = inputs.get(&builder.name).unwrap_or(&default);
            (builder.name.clone(), builder.to_module(inputs))
        })
        .collect()
}

fn push_button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut pulse_q = VecDeque::new();
    let (mut low, mut high) = (0, 0);
    pulse_q.push_back(Pulse {
        from: "Elves".to_string(),
        to: "broadcaster".to_string(),
        is_high: false,
    });

    while let Some(pulse) = pulse_q.pop_front() {
        if pulse.is_high {
            high += 1;
        } else {
            low += 1;
        };

        if let Some(module) = modules.get_mut(&pulse.to) {
            for p2 in module.recv(pulse) {
                pulse_q.push_back(p2)
            }
        } else {
            //println!("Module does not exist: {}", pulse.to);
        }
    }

    (low, high)
}

#[derive(Debug)]
struct Module {
    name: String,
    dest: Vec<String>,
    variant: Variant,
}

impl Module {
    fn recv(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let output = match &mut self.variant {
            Variant::Broadcast => Some(pulse.is_high),
            Variant::FlipFlop(f) => f.recv(pulse),
            Variant::Conjunction(c) => c.recv(pulse),
        };

        if let Some(is_high) = output {
            self.dest
                .iter()
                .map(|dest| Pulse {
                    from: self.name.to_owned(),
                    to: dest.to_owned(),
                    is_high,
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug)]
enum Variant {
    Broadcast,
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

#[derive(Debug)]
struct FlipFlop(bool);

impl FlipFlop {
    fn recv(&mut self, pulse: Pulse) -> Option<bool> {
        if pulse.is_high {
            None
        } else {
            self.0 = !self.0;
            Some(self.0)
        }
    }
}

#[derive(Debug)]
struct Conjunction(HashMap<String, bool>);

impl Conjunction {
    fn recv(&mut self, pulse: Pulse) -> Option<bool> {
        self.0.insert(pulse.from, pulse.is_high);

        if self.0.values().all(|v| *v) {
            Some(false)
        } else {
            Some(true)
        }
    }
}

#[derive(Debug)]
struct Pulse {
    from: String,
    to: String,
    is_high: bool,
}

struct ModuleBuilder {
    name: String,
    dest: Vec<String>,
    prefix: char,
}

impl ModuleBuilder {
    fn to_module(self, inputs: &[String]) -> Module {
        let variant = match self.prefix {
            '%' => Variant::FlipFlop(FlipFlop(false)),
            '&' => Variant::Conjunction(Conjunction(
                inputs.iter().map(|input| (input.clone(), false)).collect(),
            )),
            '_' => Variant::Broadcast,
            _ => panic!("Unexpected prefix"),
        };

        Module {
            name: self.name,
            dest: self.dest,
            variant,
        }
    }
}

use std::collections::{HashMap, VecDeque};

use regex::Regex;

use crate::solution::Solution;
inventory::submit!(Solution::new(20, 1, p1));
inventory::submit!(Solution::new(20, 2, p2));
