pub fn p1(input: &str) -> String {
    let digits: Vec<Vec<u32>> = input
        .split_whitespace()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    println!("Digits: {:?}", digits);

    let sum: u32 = digits
        .into_iter()
        .map(|line_digits| combine_first_and_last_digit(&line_digits))
        .sum();

    format!("Sum: {}", sum)
}

pub fn p2(input: &str) -> String {
    p1(&replace_spelled_out_digits(input))
}

fn combine_first_and_last_digit(line_digits: &[u32]) -> u32 {
    let first = line_digits.first().expect("No first digit");
    let last = line_digits.last().expect("No last digit");

    first * 10 + last
}

fn replace_spelled_out_digits(input: &str) -> String {
    input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

use crate::solution::Solution;
inventory::submit!(Solution::new(1, 1, p1));
inventory::submit!(Solution::new(1, 2, p2));
