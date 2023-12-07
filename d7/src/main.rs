#![allow(dead_code)]
use std::{cmp::Ordering, io};

use itertools::Itertools;

fn main() -> io::Result<()> {
    let solution: usize = std::fs::read_to_string("./input")?
        .lines()
        .flat_map(|l| l.split_once(' '))
        .sorted_by(|(a, _), (b, _)| cmp_hands(a, b)) // Part 1
        // .sorted_by(|(a, _), (b, _)| cmp_hands2(a, b)) // Part 2
        .flat_map(|(_, score)| score.parse::<u32>())
        .enumerate()
        .map(|(i, score)| (i + 1) * score as usize)
        .sum();

    println!("Solution: {solution}");
    Ok(())
}

fn cmp_hands(a: &str, b: &str) -> Ordering {
    match hand_type(a).cmp(&hand_type(b)) {
        Ordering::Equal => a
            .chars()
            .zip(b.chars())
            .map(|(x, y)| char_to_value(x).cmp(&char_to_value(y)))
            .skip_while(|o| o.is_eq())
            .next()
            .unwrap(),
        o => o,
    }
}

fn hand_type(hand: &str) -> u8 {
    let counts = hand.chars().counts();
    let mut it = counts.values().sorted().rev().take(2);
    match (it.next().unwrap(), it.next().unwrap_or(&0)) {
        (5, _) => 8,
        (4, _) => 7,
        (3, 2) => 6,
        (3, _) => 5,
        (2, 2) => 4,
        (2, _) => 3,
        (1, _) => 2,
        _ => 0,
    }
}

fn char_to_value(c: char) -> u8 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn cmp_hands2(a: &str, b: &str) -> Ordering {
    match hand_type2(a).cmp(&hand_type2(b)) {
        Ordering::Equal => a
            .chars()
            .zip(b.chars())
            .map(|(x, y)| char_to_value2(x).cmp(&char_to_value2(y)))
            .skip_while(|o| o.is_eq())
            .next()
            .unwrap(),
        o => o,
    }
}

fn hand_type2(hand: &str) -> u8 {
    let mut counts = hand.chars().counts();
    let jockers = counts.remove(&'J').unwrap_or(0);
    let mut it = counts.values().sorted().rev().take(2);
    match (it.next().unwrap_or(&0) + jockers, it.next().unwrap_or(&0)) {
        (5, _) => 8,
        (4, _) => 7,
        (3, 2) => 6,
        (3, _) => 5,
        (2, 2) => 4,
        (2, _) => 3,
        (1, _) => 2,
        _ => 0,
    }
}

fn char_to_value2(c: char) -> u8 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}
