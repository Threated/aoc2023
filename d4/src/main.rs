use std::{collections::HashSet, io};

fn main() -> io::Result<()> {
    let scores = std::fs::read_to_string("./input")?
        .lines()
        .map(count_winning)
        .collect::<Vec<_>>();
    println!("Part 1: {}", part1(&scores));
    println!("Part 2: {}", part2(&scores));
    Ok(())
}

fn part1(scores: &Vec<usize>) -> usize {
    scores
        .iter()
        .map(|&winning_nums| {
            if winning_nums < 2 {
                winning_nums
            } else {
                2_usize.pow((winning_nums - 1) as u32)
            }
        })
        .sum()
}

fn part2(scores: &Vec<usize>) -> usize {
    let mut copies = vec![1; scores.len()];
    scores.iter().enumerate().for_each(|(i, wins)| {
        let cards_per_win = copies[i];
        copies[i + 1..i + wins + 1]
            .iter_mut()
            .for_each(|v| *v += cards_per_win)
    });
    copies.iter().sum()
}

fn count_winning(game: &str) -> usize {
    game.split_once(':')
        .unwrap()
        .1
        .trim()
        .split(" | ")
        .map(|v| v.split(' ').flat_map(str::parse).collect::<HashSet<u32>>())
        .collect::<Vec<_>>()
        .try_into()
        .map(|[a, b]: [_; 2]| a.intersection(&b).count())
        .unwrap_or(0)
}
