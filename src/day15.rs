pub fn p1(input: &str) -> String {
    let s: u32 = input.trim().split(',').map(|s| hash(s.chars())).sum();
    format!("Sum: {}", s)
}

pub fn p2(input: &str) -> String {
    let steps: Vec<(String, Op)> = input.trim().split(',').map(parse_step).collect();
    let mut hashmap: Vec<Vec<(String, u32)>> = (0..256).map(|_| Vec::new()).collect();

    for (label, op) in steps {
        match op {
            Op::Remove => remove(&mut hashmap, &label),
            Op::Insert(val) => insert(&mut hashmap, label, val),
        }
    }

    let sum: usize = hashmap
        .into_iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(move |(j, entry)| (i + 1) * (j + 1) * entry.1 as usize)
        })
        .sum();

    format!("Sum: {}", sum)
}

fn hash(chars: impl IntoIterator<Item = char>) -> u32 {
    chars
        .into_iter()
        .map(|c| c as u32)
        .fold(0, |acc, v| ((acc + v) * 17) % 256)
}

fn insert(map: &mut [Vec<(String, u32)>], label: String, val: u32) {
    let idx = hash(label.chars()) as usize;
    let entry = map.get_mut(idx).expect("Impossibru");

    if let Some((box_idx, _)) = entry.iter().enumerate().find(|(_, l2)| label == l2.0) {
        entry[box_idx].1 = val;
    } else {
        entry.push((label, val))
    }
}

fn remove(map: &mut [Vec<(String, u32)>], label: &str) {
    let idx = hash(label.chars()) as usize;
    let entry = map.get_mut(idx).expect("Impossibru");

    if let Some((box_idx, _)) = entry.iter().enumerate().find(|(_, l2)| label == l2.0) {
        entry.remove(box_idx);
    }
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
                .expect("Focal length is not digit"),
        ),
        _ => panic!("Unexpected op"),
    };

    (label, op)
}

#[derive(Debug, Clone)]
enum Op {
    Remove,
    Insert(u32),
}

use crate::solution::Solution;
inventory::submit!(Solution::new(15, 1, p1));
inventory::submit!(Solution::new(15, 2, p2));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashing_works() {
        assert_eq!(hash("HASH".chars()), 52)
    }
}
