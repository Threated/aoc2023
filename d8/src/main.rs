use std::{io, ops::ControlFlow, collections::HashMap};

fn main() -> io::Result<()> {
    let input = std::fs::read_to_string("./input")?;
    let (path, map) = input.split_once("\r\n\r\n").unwrap();
    let map = map
        .split("\r\n")
        .flat_map(|s| s.split_once(" = "))
        .flat_map(|v| Some((v.0, v.1[1..v.1.len() - 1].split_once(", ")?)))
        .collect::<HashMap<_, _>>();
    let ControlFlow::Break(part1) = path
        .chars()
        .cycle()
        .try_fold(("AAA", 0), |(key, counter), chr| {
            let entry = map.get(key).unwrap();
            let next = if chr == 'L' { entry.0 } else { entry.1 };
            if next == "ZZZ" {
                ControlFlow::Break(counter + 1)
            } else {
                ControlFlow::Continue((next, counter + 1))
            }
        }) else { unreachable!() };
    println!("Part 1: {part1}");

    let ends: Vec<usize> = map.keys().filter(|k| k.ends_with('A')).map(|start| {
        let ControlFlow::Break(count) = path
            .chars()
            .cycle()
            .try_fold((*start, 0), |(key, counter), chr| {
                let entry = map.get(key).expect(&format!("Failed to lookup key: {key}"));
                let next = if chr == 'L' { entry.0 } else { entry.1 };
                if next.ends_with('Z') {
                    ControlFlow::Break(counter + 1)
                } else {
                    ControlFlow::Continue((next, counter + 1))
                }
            }) else { unreachable!() };
        count
        }).collect();
    let part2 = lcm(&ends);
    println!("Part 2: {part2}");
    Ok(())
}

// Had to look this stuff up for part 2 https://github.com/ChristopherBiscardi/advent-of-code/blob/b8ce6663f219d22b4fd9194030e89cf278113714/2023/rust/day-08/src/part2.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}