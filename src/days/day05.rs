use std::cmp;
use std::str::FromStr;
use itertools::{Itertools, PeekingNext};
use crate::util::Solution;


type Num = u64;

#[derive(Debug)]
struct Range {
    src: Num,
    dst: Num,
    len: Num,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let dst = s.next().unwrap().parse::<Num>().unwrap();
        let src = s.next().unwrap().parse::<Num>().unwrap();
        let len = s.next().unwrap().parse::<Num>().unwrap();
        Ok(Range { src, dst, len })
    }
}

#[derive(Debug)]
pub struct Map {
    src: String,
    dst: String,
    ranges: Vec<Range>,
}

const ORDER: [&str; 7] = [
    "seed", "soil", "fertilizer",
    "water", "light", "temperature",
    "humidity"
];

#[derive(Debug)]
pub struct World {
    maps: Vec<Map>,
    seeds: Vec<Num>,
}

impl Map {
    fn from_str(s: Vec<String>) -> Self {
        let mut s = s.into_iter();
        let map_dsc = s.next().unwrap();
        let mut map_dsc = map_dsc
            .split_whitespace().next().unwrap()
            .split('-');
        let src = map_dsc.next().unwrap().to_string();
        let dst = map_dsc.skip(1).next().unwrap().to_string();

        let ranges = s.into_iter()
            .map(|line| Range::from_str(line.as_str()).unwrap())
            .sorted_by(|a, b| a.src.cmp(&b.src))
            .collect();

        Map { src, dst, ranges }
    }

    fn translate(&self, input: Num) -> Num {
        for range in self.ranges.iter() {
            if range.src <= input && input < range.src + range.len {
                return range.dst + input - range.src;
            }
        }
        input
    }
}

pub struct Day;


impl Day {
    fn group_seeds(seeds: Vec<Num>) -> Vec<(Num, Num)> {
        let mut starts = Vec::new();
        let mut ends = Vec::new();
        seeds.chunks(2).for_each(|chunk| {
            starts.push(chunk[0]);
            ends.push(chunk[0] + chunk[1]);
        });
        starts.sort();
        ends.sort();
        let mut starts = starts.into_iter().peekable();
        let mut ends = ends.into_iter().peekable();

        let mut optim_ranges = Vec::new();
        let mut open = 0;
        let mut range = Vec::new();
        loop {
            let start = starts.peek();
            let end = ends.peek();
            if start.is_none() {
                optim_ranges.push((range[0], ends.last().unwrap()));
                break;
            } else if start < end {
                let tmp = starts.next().unwrap();
                if open == 0 {
                    range.push(tmp);
                }
                open += 1;
            } else {
                open -= 1;
                let tmp = ends.next().unwrap();
                if open == 0 {
                    range.push(tmp);
                    optim_ranges.push((range[0], range[1]));
                    range.clear();
                }
            }
        }
        optim_ranges
    }
}

impl<'a> Solution<'a> for Day {
    type Input = World;
    type Output = Num;
    const DAY: &'a str = "Day05";

    fn part1(input: &Self::Input) -> Self::Output {
        let seeds: Vec<_> = input.seeds.iter()
            .map(|seed| {
                let mut seed = seed.clone();
                for map in input.maps.iter() {
                    seed = map.translate(seed);
                }
                seed
            }).collect();
        *seeds.iter().min().unwrap()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let seeds_ranges = Self::group_seeds(input.seeds.clone());
        let mut min_seed = Num::MAX;
        for seed_range in seeds_ranges.iter() {
            for seed in seed_range.0..seed_range.1 {
                let mut seed = seed;
                for map in input.maps.iter() {
                    seed = map.translate(seed);
                }
                min_seed = cmp::min(min_seed, seed);
            }
        }
        min_seed
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        let seeds_str = raw_input.get(0).unwrap();
        let seeds = seeds_str.split(':').skip(1)
            .map(|seeds| seeds.split_whitespace())
            .flatten()
            .map(|seed| seed.parse::<Num>().unwrap())
            .collect();
        let ranges = raw_input[2..].split(|line| line.is_empty())
            .map(|lines| Map::from_str(lines.to_vec()))
            .collect();
        World { seeds, maps: ranges }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_example_t1() {
        let input = get_input();
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), 35);
    }

    #[test]
    fn test_example_t2() {
        let input = get_input();
        let input = Day::parse_input(&input);
        assert_eq!(Day::part2(&input), 46);
    }

    #[test]
    fn test_intervals() {
        let input = vec![1, 2, 2, 4, 8, 2, 15, 3];
        let input = Day::group_seeds(input);
        assert_eq!(input, vec![(1, 6), (8, 10), (15, 18)]);
    }
}