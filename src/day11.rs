pub fn p1(input: &str) -> String {
    solve(input, 2)
}

pub fn p2(input: &str) -> String {
    solve(input, 1000000)
}

fn solve(input: &str, expansion_factor: usize) -> String {
    let stars: Vec<Pos> = crate::parse::char_grid_iter::<char>(input)
        .filter(|(_, _, c)| *c == '#')
        .map(|(x, y, _)| Pos::new(x as f64, y as f64))
        .collect();

    let stars = expand_stars(stars, |s| &mut s.re, expansion_factor as f64);
    let stars = expand_stars(stars, |s| &mut s.im, expansion_factor as f64);

    let sum: f64 = stars
        .iter()
        .flat_map(|s1| stars.iter().map(move |s2| (*s1, *s2)))
        .filter(|pair| pair.0 != pair.1)
        .map(manhattan_distance)
        .sum();

    format!("Sum: {}", sum / 2.)
}

fn expand_stars(mut stars: Vec<Pos>, key: impl Fn(&mut Pos) -> &mut f64, factor: f64) -> Vec<Pos> {
    stars.sort_by_key(|star| *key(&mut star.clone()) as usize);

    stars
        .into_iter()
        .fold((0., Pos::new(0., 0.), Vec::new()), |mut acc, mut star| {
            let old = star;
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

fn manhattan_distance(pair: (Pos, Pos)) -> f64 {
    (pair.0.im - pair.1.im).abs() + (pair.0.re - pair.1.re).abs()
}

type Pos = Complex64;

use num::complex::Complex64;

use crate::solution::Solution;
inventory::submit!(Solution::new(11, 1, p1));
inventory::submit!(Solution::new(11, 2, p2));
