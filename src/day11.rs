pub fn p1(input: &str) -> String {
    solve(input, 2)
}

pub fn p2(input: &str) -> String {
    solve(input, 1000000)
}

fn solve(input: &str, expansion_factor: usize) -> String {
    let stars: Vec<Pos> = input
        .trim()
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(im, line)| {
            line.trim().chars().enumerate().filter_map(move |(re, c)| {
                if c == '#' {
                    Some(Pos::new(re as f64, im as f64))
                } else {
                    None
                }
            })
        })
        .collect();

    let stars = expand_stars(stars, |s| &mut s.re, expansion_factor as f64);
    let stars = expand_stars(stars, |s| &mut s.im, expansion_factor as f64);

    let sum: f64 = stars
        .iter()
        .flat_map(|s1| {
            stars.iter().map(move |s2| {
                if s1 != s2 {
                    (s2.im - s1.im).abs() + (s2.re - s1.re).abs()
                } else {
                    0.
                }
            })
        })
        .sum();

    format!("Sum: {}", sum / 2.)
}

fn expand_stars(mut stars: Vec<Pos>, key: impl Fn(&mut Pos) -> &mut f64, factor: f64) -> Vec<Pos> {
    stars.sort_by_key(|star| *key(&mut star.clone()) as usize);

    stars
        .into_iter()
        .fold((0., Pos::new(0., 0.), Vec::new()), |mut acc, mut star| {
            let old = star.clone();
            let to_expand = key(&mut star);
            let expansion = (*to_expand - *key(&mut acc.1) - 1.).max(0.) * (factor - 1.);

            acc.0 += expansion;
            acc.1 = old;
            *to_expand += acc.0;

            acc.2.push(star);

            acc
        })
        .2
}

type Pos = Complex64;

use num::complex::Complex64;

use crate::solution::Solution;
inventory::submit!(Solution::new(11, 1, p1));
inventory::submit!(Solution::new(11, 2, p2));
