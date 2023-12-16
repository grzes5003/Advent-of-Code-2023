use std::fmt::{Debug, Formatter};
use itertools::Itertools;
use crate::util::Solution;


#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
struct Pos(isize, isize);

impl Pos {
    fn distance(&self, other: &Self) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

pub struct Space {
    galaxies: Vec<Pos>
}

impl Space {
    fn expand(&mut self, expansion: isize) {
        let mut new_galaxies = self.galaxies.clone();
        new_galaxies.sort_by(|a, b| a.0.cmp(&b.0));
        let last_idx = new_galaxies.len() - 1;
        let mut add = 0;
        for idx in 0..last_idx {
            let diff = std::cmp::max(new_galaxies[idx+1].0 - new_galaxies[idx].0 - 1, 0);
            new_galaxies[idx].0 += add;
            add += diff * (expansion - 1);
        }
        new_galaxies[last_idx].0 += add;

        new_galaxies.sort_by(|a, b| a.1.cmp(&b.1));
        let mut add = 0;
        for idx in 0..last_idx {
            let diff = std::cmp::max(new_galaxies[idx+1].1 - new_galaxies[idx].1 - 1, 0);
            new_galaxies[idx].1 += add;
            add += diff * (expansion - 1);
        }
        new_galaxies[last_idx].1 += add;
        self.galaxies = new_galaxies;
    }

    fn max_x(&self) -> isize {
        self.galaxies.iter().map(|p| p.0).max().unwrap()
    }

    fn min_x(&self) -> isize {
        self.galaxies.iter().map(|p| p.0).min().unwrap()
    }

    fn max_y(&self) -> isize {
        self.galaxies.iter().map(|p| p.1).max().unwrap()
    }

    fn min_y(&self) -> isize {
        self.galaxies.iter().map(|p| p.1).min().unwrap()
    }
}

impl Debug for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in self.min_y()..=self.max_y() {
            for x in self.min_x()..=self.max_x() {
                if self.galaxies.contains(&Pos(x, y)) {
                    s.push('#');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

pub struct Day;

impl<'a> Solution<'a> for Day {
    type Input = Space;
    type Output = isize;
    const DAY: &'a str = "Day11";

    fn part1(input: &Self::Input) -> Self::Output {
        let mut input = Space {
            galaxies: input.galaxies.clone()
        };
        input.expand(2);
        input.galaxies.iter().combinations(2)
            .map(|pair| pair[0].distance(pair[1]))
            .sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut input = Space {
            galaxies: input.galaxies.clone()
        };
        input.expand(1_000_000);
        input.galaxies.iter().combinations(2)
            .map(|pair| pair[0].distance(pair[1]))
            .sum()
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        let mut galaxies = Vec::new();
        for y in 0..raw_input.len() {
            for x in 0..raw_input[y].len() {
                let c = raw_input[y].chars().nth(x).unwrap();
                if c == '#' {
                    galaxies.push(Pos(x as isize, y as isize));
                }
            }
        }
        Space { galaxies }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".lines().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        let input = &Day::parse_input(&get_input());
        assert_eq!(Day::part1(input), 374);
    }

    #[test]
    fn test_part2() {
        let input = &Day::parse_input(&get_input());
        assert_eq!(Day::part2(input), 8410);
    }
}