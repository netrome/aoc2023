pub fn p1(input: &str) -> String {
    let s: u32 = input.trim().split(",").map(|s| hash(s.chars())).sum();
    format!("Sum: {}", s)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

fn hash(chars: impl IntoIterator<Item = char>) -> u32 {
    chars
        .into_iter()
        .map(|c| c as u32)
        .fold(0, |acc, v| ((acc + v) * 17) % 256)
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
