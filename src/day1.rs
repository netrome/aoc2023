pub fn p1(input: &str) -> String {
    let digits: Vec<Vec<u32>> = input
        .split_whitespace()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let sum: u32 = digits
        .into_iter()
        .map(|line_digits| combine_first_and_last_digit(&line_digits))
        .sum();

    format!("Sum: {}", sum)
}

pub fn p2(input: &str) -> String {
    let digits: Vec<Vec<u32>> = input
        .split_whitespace()
        .map(|line| line.chars())
        .map(find_digits)
        .collect();

    println!("Digits: {:?}", digits);

    let sum: u32 = digits
        .into_iter()
        .map(|line_digits| combine_first_and_last_digit(&line_digits))
        .sum();

    format!("Sum: {}", sum)
}

fn combine_first_and_last_digit(line_digits: &[u32]) -> u32 {
    let first = line_digits.first().expect("No first digit");
    let last = line_digits.last().expect("No last digit");

    first * 10 + last
}

fn find_digits(chars: impl IntoIterator<Item = char>) -> Vec<u32> {
    let mut seen = Vec::new();
    let mut digits = Vec::new();

    for char in chars {
        if let Some(digit) = char.to_digit(10) {
            digits.push(digit);
            seen = Vec::new();
        } else {
            seen.push(char)
        }

        if !seen.is_empty() {
            for start in 0..seen.len() - 1 {
                if let Some(digit) = parse_digit(&seen[start..seen.len()]) {
                    digits.push(digit);
                    seen.clear();
                    break;
                }
            }
        }
    }

    digits
}

fn parse_digit(chars: &[char]) -> Option<u32> {
    match chars {
        ['o', 'n', 'e'] => Some(1),
        ['t', 'w', 'o'] => Some(2),
        ['t', 'h', 'r', 'e', 'e'] => Some(3),
        ['f', 'o', 'u', 'r'] => Some(4),
        ['f', 'i', 'v', 'e'] => Some(5),
        ['s', 'i', 'x'] => Some(6),
        ['s', 'e', 'v', 'e', 'n'] => Some(7),
        ['e', 'i', 'g', 'h', 't'] => Some(8),
        ['n', 'i', 'n', 'e'] => Some(9),
        _ => None,
    }
}

use crate::solution::Solution;
inventory::submit!(Solution::new(1, 1, p1));
inventory::submit!(Solution::new(1, 2, p2));
