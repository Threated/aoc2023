use d1::*;
use std::io;

fn main() -> io::Result<()> {
    let solution: u32 = std::fs::read_to_string("./input")?
        .lines()
        .filter_map(str_to_num_p1)
        .sum();
    println!("Solution: {solution}");
    Ok(())
}
