use std::io;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let dim: Vec<Vec<_>> = std::fs::read_to_string("./input")?
        .lines()
        .map(str::chars)
        .map(Iterator::collect)
        .collect();
    let vertical_expansions = (0..dim[0].len())
        .into_iter()
        .filter(|i| dim.iter().all(|row| row[*i] == '.'))
        .collect_vec();

    const EXPANSION_FACTOR: isize = 1_000_000; // 2 for part 1
    let solution: u128 = dim
        .iter()
        .scan(0_isize, |acc, line| {
            if line.iter().all(|c| *c == '.') {
                *acc += EXPANSION_FACTOR;
                return Some(Vec::with_capacity(0));
            }
            *acc += 1;
            Some(
                line.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == '#')
                    .map(|(i, _)| {
                        (
                            i as isize
                                + vertical_expansions.iter().filter(|e| **e < i).count() as isize
                                    * (EXPANSION_FACTOR - 1),
                            *acc - 1,
                        )
                    })
                    .collect(),
            )
        })
        .flatten()
        .combinations(2)
        .map(|v| calc_distance(v[0], v[1]))
        .sum();
    println!("Solution: {solution}");
    Ok(())
}

fn calc_distance((a_x, a_y): (isize, isize), (b_x, b_y): (isize, isize)) -> u128 {
    ((a_x - b_x).abs() + (a_y - b_y).abs()) as u128
}

#[test]
fn asdf() {
    dbg!(calc_distance((4, 1), (9, 11)));
    dbg!(calc_distance((0, 11), (5, 11)));
}
