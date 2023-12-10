use std::{collections::HashSet, io};

fn main() -> io::Result<()> {
    let maze: Vec<_> = std::fs::read_to_string("./input")?
        .lines()
        .enumerate()
        .map(|(i, l)| parse_line(i, l))
        .collect();
    let points = main_loop_points(&maze);
    let part_1 = (points.len() as f64 / 2.).ceil() as u32;
    let part_2 = count_inside(&maze, &points);
    println!("{part_1}");
    println!("{part_2}");
    Ok(())
}

fn count_inside(maze: &Vec<Vec<Tile>>, points: &HashSet<&Tile>) -> u32 {
    maze.iter().map(|v| count_line(v, points)).sum()
}

fn count_line(line: &Vec<Tile>, points: &HashSet<&Tile>) -> u32 {
    line.iter()
        .fold((0, false), |(count, is_inside), tile| match tile {
            t @ Tile::Pipe(Pipe { chr, .. }) if points.contains(t) => (
                count,
                match chr {
                    '|' => !is_inside,
                    '7' => !is_inside,
                    'F' => !is_inside,
                    'L' => is_inside,
                    'J' => is_inside,
                    '-' => is_inside,
                    _ => unreachable!(),
                },
            ),
            // This is unsound. Actually depends on what type of outlets this S has
            Tile::Start { .. } => (count, is_inside),
            _ => {
                if is_inside {
                    (count + 1, is_inside)
                } else {
                    (count, is_inside)
                }
            }
        })
        .0
}

fn main_loop_points(maze: &Vec<Vec<Tile>>) -> HashSet<&Tile> {
    let start @ Tile::Start { x: start_x, y: start_y } = maze
        .iter()
        .flat_map(|row| row.iter().filter(|t| matches!(t, Tile::Start { .. })))
        .next()
        .unwrap() else { unreachable!() };
    use Direction::*;
    let mut next_direction = [North, South, West, East]
        .into_iter()
        .find(|d| {
            let (y_off, x_off) = d.offset();
            maze.get((*start_y as isize + y_off) as usize)
                .and_then(|row| row.get((*start_x as isize + x_off) as usize))
                .is_some_and(Tile::not_empty)
        })
        .unwrap();
    let (next_y, next_x) = next_direction.offset();
    let current_pos = (
        (*start_x as isize + next_x) as usize,
        (*start_y as isize + next_y) as usize,
    );
    std::iter::successors(Some(current_pos), |current_pos| {
        next_direction = match maze[current_pos.1][current_pos.0] {
            Tile::Pipe(Pipe { connections, .. }) => *connections
                .iter()
                .filter(|d| **d != next_direction.oposite())
                .next()
                .unwrap(),
            Tile::Empty => unreachable!(),
            Tile::Start { .. } => return None,
        };
        let (next_y, next_x) = next_direction.offset();
        Some((
            (current_pos.0 as isize + next_x) as usize,
            (current_pos.1 as isize + next_y) as usize,
        ))
    })
    .map(|(x, y)| &maze[y][x])
    .chain(std::iter::once(start))
    .collect()
}

fn parse_line(y: usize, l: &str) -> Vec<Tile> {
    l.chars()
        .enumerate()
        .map(|(x, c)| {
            use Direction::*;
            let connections = match c {
                '|' => [South, North],
                '-' => [West, East],
                'L' => [North, East],
                'J' => [North, West],
                '7' => [South, West],
                'F' => [South, East],
                '.' => return Tile::Empty,
                'S' => return Tile::Start { x, y },
                _ => unreachable!("Unknown char {c}"),
            };
            Tile::Pipe(Pipe {
                connections,
                chr: c,
                pos: (x, y),
            })
        })
        .collect()
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Tile {
    Pipe(Pipe),
    Empty,
    Start { x: usize, y: usize },
}

impl Tile {
    fn not_empty(&self) -> bool {
        !matches!(self, Tile::Empty)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Pipe {
    chr: char,
    pos: (usize, usize),
    connections: [Direction; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn oposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    fn offset(self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }
}
