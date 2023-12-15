pub fn p1(input: &str) -> String {
    let s: u32 = input.trim().split(',').map(|s| s.hash() as u32).sum();
    format!("Sum: {}", s)
}

pub fn p2(input: &str) -> String {
    let steps: Vec<(String, Op)> = input.trim().split(',').map(parse_step).collect();
    let mut map: HashMap<_, _, 5> = HashMap::new();

    for (label, op) in steps {
        match op {
            Op::Remove => map.remove(&label),
            Op::Insert(val) => map.insert(label, val).unwrap(),
        };
    }

    let sum: usize = map
        .arrays()
        .into_iter()
        .enumerate()
        .flat_map(|(i, array)| {
            array
                .values()
                .enumerate()
                .map(move |(j, val)| (i + 1) * (j + 1) * *val as usize)
        })
        .sum();

    format!("Sum: {}", sum)
}

fn parse_step(s: &str) -> (String, Op) {
    let (label, op_chars): (String, String) = s.trim().chars().partition(|c| c.is_alphabetic());

    let op = match op_chars.chars().next().expect("Empty op chars") {
        '-' => Op::Remove,
        '=' => Op::Insert(
            op_chars
                .chars()
                .nth(1)
                .expect("No focal length")
                .to_digit(10)
                .expect("Focal length is not digit") as u8,
        ),
        _ => panic!("Unexpected op"),
    };

    (label, op)
}

#[derive(Debug, Clone)]
enum Op {
    Remove,
    Insert(u8),
}

use crate::{hashmap::Hash, hashmap::HashMap, solution::Solution};
inventory::submit!(Solution::new(15, 1, p1));
inventory::submit!(Solution::new(15, 2, p2));
