pub fn p1(input: &str) -> String {
    let grid = CharGrid::from_input(input);
    let sum: u32 = grid
        .find_numbers()
        .into_iter()
        .filter(|number| grid.is_adjacent_to_symbol(number))
        .map(|number| number.value)
        .sum();

    format!("Part number sum: {:?}", sum)
}

pub fn p2(_input: &str) -> String {
    todo!();
}

#[derive(Debug)]
struct Number {
    pos: Pos,
    length: usize,
    value: u32,
}

impl Number {
    fn neighbors(&self) -> Vec<Pos> {
        let len = self.length as i32;

        let left = [
            self.pos.add_x(-1),
            self.pos.add_x(-1).add_y(1),
            self.pos.add_x(-1).add_y(-1),
        ];

        let right = [
            self.pos.add_x(len),
            self.pos.add_x(len).add_y(1),
            self.pos.add_x(len).add_y(-1),
        ];

        (0..len)
            .flat_map(|offset| {
                [
                    self.pos.add_x(offset).add_y(1),
                    self.pos.add_x(offset).add_y(-1),
                ]
            })
            .chain(left)
            .chain(right)
            .collect()
    }
}

#[derive(Debug)]
struct CharGrid {
    grid: HashMap<Pos, char>,
    width: usize,
    height: usize,
}

impl CharGrid {
    fn from_input(input: &str) -> Self {
        let width = input.lines().next().unwrap().trim().len();
        let height = input.lines().count();
        let grid = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.trim().chars().enumerate().map(move |(col, char)| {
                    (
                        Pos {
                            x: col as i32,
                            y: row as i32,
                        },
                        char,
                    )
                })
            })
            .collect();

        Self {
            width,
            height,
            grid,
        }
    }

    fn find_numbers(&self) -> Vec<Number> {
        (0..self.height as i32)
            .flat_map(|line| self.find_numbers_on_line(line))
            .collect()
    }

    fn find_numbers_on_line(&self, line: i32) -> Vec<Number> {
        let mut numbers = Vec::new();

        let mut cursor = Pos { x: 0, y: line };

        while let Some(mut c) = self.grid.get(&cursor).cloned() {
            let pos = cursor.clone();

            let mut digits = Vec::new();
            let mut length = 0;
            while let Some(digit) = c.to_digit(10) {
                digits.push(digit);
                cursor = cursor.add_x(1);
                c = self.get(&cursor);
                length += 1;
            }

            if length > 0 {
                let value = digits
                    .into_iter()
                    .rev()
                    .enumerate()
                    .map(|(idx, digit)| 10_u32.pow(idx as u32) * digit)
                    .sum();

                let number = Number { pos, length, value };

                numbers.push(number);
            }

            cursor = cursor.add_x(1);
        }

        numbers
    }

    fn is_adjacent_to_symbol(&self, number: &Number) -> bool {
        number
            .neighbors()
            .into_iter()
            .any(|pos| self.get(&pos) != '.')
    }

    fn get(&self, pos: &Pos) -> char {
        *self.grid.get(pos).unwrap_or(&'.')
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn add_x(self, x: i32) -> Self {
        Self {
            x: self.x + x,
            ..self
        }
    }

    fn add_y(self, y: i32) -> Self {
        Self {
            y: self.y + y,
            ..self
        }
    }

    fn neighbors(self) -> [Self; 8] {
        [
            self.add_x(1),
            self.add_x(1).add_y(1),
            self.add_x(1).add_y(-1),
            self.add_x(-1),
            self.add_x(-1).add_y(1),
            self.add_x(-1).add_y(-1),
            self.add_y(1),
            self.add_y(-1),
        ]
    }
}

use std::collections::HashMap;

use crate::solution::Solution;
inventory::submit!(Solution::new(3, 1, p1));
inventory::submit!(Solution::new(3, 2, p2));
