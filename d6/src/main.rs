use std::io;

use num_bigfloat::{BigFloat, TWO};

fn main() -> io::Result<()> {
    let binding = std::fs::read_to_string("./input")?;
    let mut it = binding
        .lines()
        .map(|line| line.split_whitespace().skip(1).flat_map(str::parse::<u64>));
    let part1: u64 = it
        .next()
        .into_iter()
        .flatten()
        .zip(it.next().into_iter().flatten())
        .map(solve_for_x)
        .product();
    println!("Part1: {part1}");
    println!("Part2: {}", solve_for_x((46807866, 214117714021024)));
    Ok(())
}

fn solve_for_x((t, d): (u64, u64)) -> u64 {
    let root = BigFloat::from(t.pow(2) - 4 * d).sqrt();
    let x1 = (-BigFloat::from(t) + root) / -TWO;
    let x2 = (-BigFloat::from(t) - root) / -TWO;
    // hack: For the case where the numbers match
    let break_even_numbers = BigFloat::from(0.000001);
    ((x1 + break_even_numbers).ceil() - (x2 - break_even_numbers).ceil())
        .abs()
        .to_u64()
        .unwrap()
}
