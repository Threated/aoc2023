use std::io;

fn main() -> io::Result<()> {
    let solution: isize = std::fs::read_to_string("./input")?
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(str::parse)
                .rev() // Part 2 toggle
                .collect::<Vec<isize>>()
        })
        .map(|vs| predict_next(&vs))
        .sum();
    println!("Solution: {solution}");
    Ok(())
}

fn predict_next(values: &[isize]) -> isize {
    values
        .iter()
        .all(|v| *v == 0)
        .then_some(0)
        .unwrap_or_else(|| {
            predict_next(&values.windows(2).map(|v| v[1] - v[0]).collect::<Vec<_>>())
                + values.last().unwrap()
        })
}
