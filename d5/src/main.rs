#![allow(dead_code)]
use std::{io, ops::Range};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() -> io::Result<()> {
    let data = std::fs::read_to_string("./input")?;
    let mut data_it = data.split("\r\n\r\n");
    let seeds_p1: Vec<usize> = data_it
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .flat_map(str::parse)
        .collect();
    let maps = data_it
        .map(|map| map.split('\n').skip(1).flat_map(MapEntry::parse).collect())
        .collect::<Vec<Vec<_>>>();

    let part_1 = seeds_p1
        .iter()
        .map(|&seed| lookup_in_maps(seed, &maps))
        .min()
        .unwrap();

    // Initial solve was without rayon and took ~2m in release mode
    // With rayon ~20s
    let part_2_slow = seeds_p1
        .chunks_exact(2)
        .map(|v| (v[0]..v[0] + v[1]).collect::<Vec<_>>())
        .flat_map(|r| r.par_iter().map(|s| lookup_in_maps(*s, &maps)).min())
        .min()
        .unwrap();
    // let part_2_fast = seeds_p1.chunks_exact(2).map(|v| (v[0]..v[0] + v[1]));

    println!("Part1: {part_1}");
    println!("Part2: {part_2_slow}");
    // println!("Part2: {part_2_fast}");
    Ok(())
}

fn lookup_range_in_maps(seeds: Range<usize>, maps: &[Vec<MapEntry>]) -> Vec<Range<usize>> {
    maps.first()
        .map(|map| {
            map.into_iter()
                .flat_map(|entry| entry.sub_ranges(&seeds).into_iter().flatten())
                .flat_map(|sub_range| lookup_range_in_maps(sub_range, &maps[1..]))
                .collect()
        })
        .unwrap_or(vec![seeds])
}

fn lookup_in_maps(seed: usize, maps: &[Vec<MapEntry>]) -> usize {
    maps.first()
        .map(|map| {
            lookup_in_maps(
                map.into_iter()
                    .flat_map(|entry| entry.check_and_convert(seed))
                    .next()
                    .unwrap_or(seed),
                &maps[1..],
            )
        })
        .unwrap_or(seed)
}

#[derive(Debug)]
struct MapEntry {
    src: usize,
    dst: usize,
    len: usize,
}

impl MapEntry {
    fn parse(input: &str) -> Option<Self> {
        let mut it = input.split_whitespace().flat_map(str::parse);
        Some(Self {
            dst: it.next()?,
            src: it.next()?,
            len: it.next()?,
        })
    }

    fn check_and_convert(&self, num: usize) -> Option<usize> {
        (self.src..self.src + self.len)
            .contains(&num)
            .then(|| num - self.src + self.dst)
    }

    fn sub_ranges(&self, other: &Range<usize>) -> [Option<Range<usize>>; 3] {
        let not_empty_then = |r: Range<_>| (r.start != r.end).then_some(r);
        // TODO
        [
            not_empty_then(self.src.min(other.start)..self.src.max(other.start)),
            not_empty_then(self.src.max(other.start)..other.end.min(self.src + self.len))
                .map(|r| r.start - self.src + self.len..r.end - self.src + self.dst),
            not_empty_then(
                (self.src + self.len).min(other.start)..(self.src + self.len).max(other.start),
            ),
        ]
    }
}
