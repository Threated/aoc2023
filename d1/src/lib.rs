use std::collections::HashMap;

pub fn str_to_num_p1(s: &str) -> Option<u32> {
    let mut it = s.chars().filter_map(|c| c.to_digit(10));
    let first = it.next()?;
    Some(first * 10 + it.last().unwrap_or(first))
}

pub fn str_to_num_p2(s: &str) -> Option<u32> {
    let map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut it = s.chars().enumerate().flat_map(|(i, c)| {
        c.to_digit(10).or(map
            .iter()
            .find_map(|(k, v)| s[i..].starts_with(k).then_some(*v)))
    });
    let first = it.next()?;
    Some(first * 10 + it.last().unwrap_or(first))
}

pub fn str_to_num_p2_fast(s: &str) -> Option<u32> {
    let map = |q: &str| {
        match q.get(..3)? {
            "one" => Some(1),
            "two" => Some(2),
            "six" => Some(6),
            _ => None,
        }
        .or_else(|| match q.get(..4)? {
            "four" => Some(4),
            "five" => Some(5),
            "nine" => Some(9),
            _ => None,
        })
        .or_else(|| match q.get(..5)? {
            "three" => Some(3),
            "seven" => Some(7),
            "eight" => Some(8),
            _ => None,
        })
    };
    let mut it = s
        .chars()
        .enumerate()
        .flat_map(|(i, c)| c.to_digit(10).or(map(&s[i..])));
    let first = it.next()?;
    Some(first * 10 + it.last().unwrap_or(first))
}
