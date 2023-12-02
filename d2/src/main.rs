#![allow(dead_code)]
use std::io;

fn main() -> io::Result<()> {
    let solution: u32 = std::fs::read_to_string("./input")?
        .lines()
        .filter_map(check_game_p2)
        .sum();
    println!("Solution: {solution}");
    Ok(())
}

#[derive(Debug)]
enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

impl Color {
    fn parse(s: &str) -> Option<Self> {
        match s.trim().split_once(' ')? {
            (num, "red") => num.parse().ok().map(Self::Red),
            (num, "green") => num.parse().ok().map(Self::Green),
            (num, "blue") => num.parse().ok().map(Self::Blue),
            _ => None,
        }
    }

    fn under_threshold(&self, (rt, gt, bt): (u32, u32, u32)) -> bool {
        match self {
            Color::Red(r) => *r <= rt,
            Color::Green(g) => *g <= gt,
            Color::Blue(b) => *b <= bt,
        }
    }
}

fn check_game_p1(s: &str) -> Option<u32> {
    let (game_id, game) = s.split_once(':')?;
    game.split(';')
        .map(|d| d.split(',').filter_map(Color::parse))
        .all(|mut draws| draws.all(|draw| draw.under_threshold((12, 13, 14))))
        .then_some(game_id.split_once(' ')?.1.parse().ok()?)
}

fn check_game_p2(s: &str) -> Option<u32> {
    s.split_once(':')?
        .1
        .split(';')
        .map(|d| {
            d.split(',')
                .filter_map(Color::parse)
                .fold([0, 0, 0], |mut acc, c| {
                    match c {
                        Color::Red(r) => acc[0] += r,
                        Color::Green(g) => acc[1] += g,
                        Color::Blue(b) => acc[2] += b,
                    };
                    acc
                })
        })
        .reduce(|[ar, ag, ab], [r, g, b]| [ar.max(r), ag.max(g), ab.max(b)])
        .map(|v| v.iter().product())
}
