use std::num::ParseIntError;
use std::str::FromStr;
use itertools::Itertools;
use thiserror::Error;
use crate::util::Solution;


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Cannot parse to int: {0}")]
    PIE(#[from] ParseIntError),
    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),
}

// R G B
#[derive(Debug)]
struct Bag(u32, u32, u32);

impl FromStr for Bag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let (mut r, mut g, mut b) = (0, 0, 0);
        for part in parts.into_iter() {
            let slices = part.trim().split(' ').collect::<Vec<_>>();
            match slices.as_slice() {
                [a, "red"] => { r = a.parse::<u32>()?; }
                [a, "green"] => { g = a.parse::<u32>()?; }
                [a, "blue"] => { b = a.parse::<u32>()?; }
                _ => Err(ParseError::UnexpectedToken(part.to_owned()))?
            }
        }
        Ok(Bag(r, g, b))
    }
}

#[derive(Debug)]
pub struct Game {
    uid: u32,
    rounds: Vec<Bag>,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let uid = parts.next().unwrap()
            .split(' ').skip(1).next().unwrap()
            .parse::<u32>().unwrap();
        let mut raw_rounds = parts.next().unwrap().split(";");
        let rounds = raw_rounds.into_iter()
            .map(|part| Bag::from_str(part))
            .collect::<Result<_, _>>()?;
        Ok(Game { uid, rounds })
    }
}

pub struct Day;

impl Day {
    const fn limit() -> Bag {
        Bag(12, 13, 14)
    }
}

impl<'a> Solution<'a> for Day {
    type Input = Vec<Game>;
    type Output = u32;
    const DAY: &'a str = "Day02";

    fn part1(input: &Self::Input) -> Self::Output {
        input.into_iter()
            .filter(|game| {
                game.rounds.iter()
                    .all(|bag| bag.0 <= Self::limit().0
                        && bag.1 <= Self::limit().1
                        && bag.2 <= Self::limit().2)
            })
            .map(|game| game.uid)
            .sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input.into_iter()
            .map(|game| {
                let (mut r, mut g, mut b) = (0, 0, 0);
                println!("{:?}", game);
                for round in game.rounds.iter() {
                    r = r.max(round.0);
                    g = g.max(round.1);
                    b = b.max(round.2);
                };
                r * g * b
            })
            .sum()
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.iter()
            .map(|line| Game::from_str(line.as_str()).unwrap())
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_t2() {
        let input =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".lines()
                .map(|line| Game::from_str(line).unwrap())
                .collect::<Vec<_>>();
        assert_eq!(Day::part2(&input), 2286);
    }
}