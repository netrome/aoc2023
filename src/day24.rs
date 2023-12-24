pub fn p1(input: &str) -> String {
    let hailstones: Vec<_> = input.trim().lines().map(HailStone::from_line).collect();
    //let min = 7.;
    //let max = 27.;
    let min = 200000000000000.;
    let max = 400000000000000.;

    let mut intersections = 0;

    for i in 0..hailstones.len() {
        for j in 0..i {
            let h1 = &hailstones[i];
            let h2 = &hailstones[j];

            let x = h1.intersection_xy(h2);

            if x.is_finite() && h1.is_future(x) && h2.is_future(x) {
                if x >= min && x <= max {
                    let y = h1.evaluate_xy(x);
                    if y >= min && y <= max {
                        intersections += 1;
                    }
                }
            }
        }
    }

    format!("Number of intersections: {}", intersections)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

#[derive(Debug, Clone)]
struct HailStone {
    pos: Vector3,
    velocity: Vector3,
}

impl HailStone {
    fn from_line(line: &str) -> Self {
        let l = line.trim().replace(" ", "");
        let (pos, velocity) = l.split_once('@').unwrap();

        let parse_vec = |s: &str| {
            let v: Vec<_> = s.split(',').map(|x| x.parse().unwrap()).collect();
            Vector3::new(v[0], v[1], v[2])
        };

        let pos = parse_vec(pos);
        let velocity = parse_vec(velocity);

        Self { pos, velocity }
    }

    // Line equation: y = y0 + (dy/dx) (x - x0)
    // y0 + (dy0/dx0) (x - x0) = y1 + (dy1/dx1) (x - x1)
    // y1 - (dy1/dx1)x1 - y0 + (dy0/dx0)x0 = (dy0/dx0 - dy1/dx1) x
    // x = (y1 - y0 + dy0x0/dx0 - dy1x1/dx1) / (dy0/dx0 - dy1/dx1)
    //
    // Returns the x value where the lines intersects
    fn intersection_xy(&self, other: &Self) -> f64 {
        let (x0, y0) = (self.pos.x, self.pos.y);
        let (dx0, dy0) = (self.velocity.x, self.velocity.y);

        let (x1, y1) = (other.pos.x, other.pos.y);
        let (dx1, dy1) = (other.velocity.x, other.velocity.y);

        (y1 - y0 + dy0 * x0 / dx0 - dy1 * x1 / dx1) / (dy0 / dx0 - dy1 / dx1)
    }

    // Find the y value at the given x
    fn evaluate_xy(&self, x: f64) -> f64 {
        let (x0, y0) = (self.pos.x, self.pos.y);
        let (dx, dy) = (self.velocity.x, self.velocity.y);

        y0 + (dy / dx) * (x - x0)
    }

    fn is_future(&self, x: f64) -> bool {
        match self.velocity.x.signum() as i64 {
            1 => x > self.pos.x,
            -1 => x < self.pos.x,
            _ => panic!("Weird velocity sign"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, derive_more::Add, derive_more::Sub)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl core::ops::Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, mut rhs: Vector3) -> Self::Output {
        rhs.x *= self;
        rhs.y *= self;
        rhs.z *= self;

        rhs
    }
}

use crate::solution::Solution;
inventory::submit!(Solution::new(24, 1, p1));
inventory::submit!(Solution::new(24, 2, p2));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_algebra() {
        let v1 = Vector3::new(13., 14., 1337.);
        let v2 = Vector3::new(1., 0.5, -2.3);
        let sum = Vector3::new(14., 14.5, 1334.7);
        let diff = Vector3::new(12., 13.5, 1339.3);
        let double = Vector3::new(26., 28., 2674.);

        assert_eq!(v1 + v2, sum);
        assert_eq!(v1 - v2, diff);
        assert_eq!(2. * v1, double);
    }

    #[test]
    fn hailstone_math_one() {
        let h1 = HailStone::from_line("19, 13, 30 @ -2, 1, -2");
        let h2 = HailStone::from_line("18, 19, 22 @ -1, -1, -2");

        assert!(close_enough(h1.intersection_xy(&h2), 14.33333333));
        assert!(close_enough(h1.evaluate_xy(14.3333333333), 15.33333333));
        assert!(close_enough(h2.evaluate_xy(14.3333333333), 15.33333333));
        assert!(h1.is_future(14.333));
        assert!(h2.is_future(14.333));
    }

    #[test]
    fn hailstone_math_two() {
        let h1 = HailStone::from_line("19, 13, 30 @ -2, 1, -2");
        let h2 = HailStone::from_line("12, 31, 28 @ -1, -2, -1");

        assert_eq!(h1.evaluate_xy(6.2), 19.4);
        assert_eq!(h2.evaluate_xy(6.2), 19.4);
        assert_eq!(h1.intersection_xy(&h2), 6.2);
        assert!(h1.is_future(6.2));
        assert!(h2.is_future(6.2));
    }

    #[test]
    fn hailstone_math_parallel() {
        let h1 = HailStone::from_line("18, 19, 22 @ -1, -1, -2");
        let h2 = HailStone::from_line("20, 25, 34 @ -2, -2, -4");

        assert!(f64::is_infinite(h1.intersection_xy(&h2)));
    }

    #[test]
    fn hailstone_math_past() {
        let h1 = HailStone::from_line("18, 19, 22 @ -1, -1, -2");
        let h2 = HailStone::from_line("20, 19, 15 @ 1, -5, -3");

        let intersection = h1.intersection_xy(&h2);
        assert!(!h1.is_future(intersection));
        assert!(!h2.is_future(intersection));
    }

    fn close_enough(f1: f64, f2: f64) -> bool {
        (f1 - f2).abs() < 0.000001
    }
}
