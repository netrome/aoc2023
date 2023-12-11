pub fn p1(input: &str) -> String {
    let mut stars: Vec<Pos> = input
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

    stars.sort_by_key(|star| star.re as usize);

    let mut stars = stars
        .into_iter()
        .fold((0., Pos::new(0., 0.), Vec::new()), |mut acc, star| {
            let expansion = (star.re - acc.1.re - 1.).max(0.);
            acc.0 += expansion;
            acc.1 = star;
            let next = Pos::new(star.re + acc.0, star.im);
            acc.2.push(next);

            acc
        })
        .2;

    stars.sort_by_key(|star| star.im as usize);

    let stars = stars
        .into_iter()
        .fold((0., Pos::new(0., 0.), Vec::new()), |mut acc, star| {
            let expansion = (star.im - acc.1.im - 1.).max(0.);
            acc.0 += expansion;
            acc.1 = star;
            let next = Pos::new(star.re, star.im + acc.0);
            acc.2.push(next);

            acc
        })
        .2;

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

pub fn p2(_input: &str) -> String {
    todo!();
}

type Pos = Complex64;

use num::complex::Complex64;

use crate::solution::Solution;
inventory::submit!(Solution::new(11, 1, p1));
inventory::submit!(Solution::new(11, 2, p2));
