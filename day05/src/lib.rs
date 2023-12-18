use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

pub struct Map {
    pub map_type: String,
    pub entries: Vec<Entry>,
}

pub struct Entry {
    pub src_start: usize,
    pub dest_start: usize,
    pub length: usize,
}

pub fn parse_input(input: &str) -> Almanac {
    let (seeds, maps) = input.split_once('\n').unwrap();

    let seeds = seeds
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let maps = maps
        .split("\n\n")
        .map(|section| {
            let (map_type, map_entries) = section.split_once(":\n").unwrap();

            let map_type = map_type.trim().to_owned();

            let entries = map_entries
                .lines()
                .map(|line| {
                    let [dest_start, src_start, length] = line
                        .split_whitespace()
                        .map(|c| c.parse().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap();

                    Entry {
                        src_start,
                        dest_start,
                        length,
                    }
                })
                .sorted_by(|a, b| a.src_start.cmp(&b.src_start))
                .collect::<Vec<_>>();

            Map { map_type, entries }
        })
        .collect();

    Almanac { seeds, maps }
}

fn get_proper_mapped_id(id: usize, ranges: &[Entry]) -> usize {
    ranges
        .iter()
        .find(|e| (e.src_start..(e.src_start + e.length)).contains(&id))
        .map_or(id, |e| {
            let src = e.src_start..(e.src_start + e.length);
            let dest = e.dest_start..(e.dest_start + e.length);

            let distance = id - src.start;

            dest.start + distance
        })
}

pub fn get_proper_min_location(seed: usize, maps: &[Map]) -> usize {
    maps.iter()
        .fold(seed, |acc, map| get_proper_mapped_id(acc, &map.entries))
}

pub mod part1;
pub mod part2;
