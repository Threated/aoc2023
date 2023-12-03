use std::{io, iter};

use itertools::Itertools;

fn main() -> io::Result<()> {
    let grid: Vec<Vec<_>> = std::fs::read_to_string("./input")?
        .lines()
        .map(str::chars)
        .map(Iterator::collect)
        .collect();
    println!("Part 1 {}", part1(&grid));
    println!("Part 2 {}", part2(&grid));
    Ok(())
}

fn part1(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().enumerate().map(move |(j, c)| ((i, j), c)))
        .flatten()
        .group_by(|v| v.1.is_digit(10))
        .into_iter()
        .filter_map(|(is_digit, mut digits)| {
            is_digit.then_some(Num {
                cords: digits.next()?.0,
                len: digits.count() + 1,
            })
        })
        .filter_map(|n| n.is_part(&grid).then(|| n.to_value(&grid)))
        .sum()
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    grid.iter()
        .enumerate()
        .map(|(i, row)| row.iter().enumerate().map(move |(j, c)| ((i, j), c)))
        .flatten()
        .group_by(|v| v.1.is_digit(10))
        .into_iter()
        .filter_map(|(is_digit, mut digits)| {
            is_digit.then_some(Num {
                cords: digits.next()?.0,
                len: digits.count() + 1,
            })
        })
        .map(|n| n.iter_parts(&grid).map(|p| (p, n)).collect::<Vec<_>>())
        .flatten()
        .sorted_by_key(|v: &((usize, usize), Num)| v.0)
        .group_by(|v| v.0)
        .into_iter()
        .filter_map(|v| {
            v.1.collect_tuple()
                .map(|(a, b): (_, _)| a.1.to_value(&grid) * b.1.to_value(&grid))
        })
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Num {
    cords: (usize, usize),
    len: usize,
}

impl Num {
    fn to_value(&self, grid: &Vec<Vec<char>>) -> usize {
        grid[self.cords.0][self.cords.1..self.cords.1 + self.len]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap()
    }

    fn is_part(&self, grid: &Vec<Vec<char>>) -> bool {
        let top_row = grid
            .get(self.cords.0.wrapping_sub(1))
            .map(|row| {
                (self.cords.1.saturating_sub(1)..self.cords.1 + self.len + 1)
                    .filter_map(|i| row.get(i))
            })
            .into_iter()
            .flatten();
        let bottom_row = grid
            .get(self.cords.0 + 1)
            .map(|row| {
                (self.cords.1.saturating_sub(1)..self.cords.1 + self.len + 1)
                    .filter_map(|i| row.get(i))
            })
            .into_iter()
            .flatten();
        let left_element = grid[self.cords.0].get(self.cords.1.wrapping_sub(1));
        let right_element = grid[self.cords.0].get(self.cords.1 + self.len);
        top_row
            .chain(bottom_row)
            .chain(left_element)
            .chain(right_element)
            .any(|c| (!c.is_digit(10) && *c != '.'))
    }

    fn iter_parts<'a>(
        &self,
        grid: &'a Vec<Vec<char>>,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        let top_row = iter::once(self.cords.0.wrapping_sub(1))
            .cartesian_product(self.cords.1.saturating_sub(1)..self.cords.1 + self.len + 1);
        let bottom_row = iter::once(self.cords.0 + 1)
            .cartesian_product(self.cords.1.saturating_sub(1)..self.cords.1 + self.len + 1);
        let left_element = (self.cords.0, self.cords.1.wrapping_sub(1));
        let right_element = (self.cords.0, self.cords.1 + self.len);
        top_row
            .chain(bottom_row)
            .chain(iter::once(left_element))
            .chain(iter::once(right_element))
            .filter(|cords| {
                grid.get(cords.0)
                    .and_then(|row| row.get(cords.1))
                    .is_some_and(|c| *c == '*')
            })
    }
}
