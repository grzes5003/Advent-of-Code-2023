use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;
use crate::days::day04::ParseError::WrongFormat;
use crate::util::Solution;


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Cannot parse to int: {0}")]
    PIE(#[from] ParseIntError),
    #[error("Wrong Format: {0}")]
    WrongFormat(String),
}

#[derive(Debug)]
pub struct Card {
    id: u32,
    win: HashSet<u8>,
    own: HashSet<u8>,
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(":");
        let id = parts.next().ok_or(WrongFormat(s.to_string()))?
            .split_whitespace()
            .skip(1)
            .next().ok_or(WrongFormat(s.to_string()))?
            .parse::<u32>()?;
        let mut numbers = parts.next().unwrap().split('|');
        let mut win = numbers.next().ok_or(WrongFormat(s.to_string()))?.split_whitespace()
            .map(|n| n.parse::<u8>())
            .collect::<Result<HashSet<_>, _>>()?;
        let mut own = numbers.next().unwrap().split_whitespace()
            .map(|n| n.parse::<u8>())
            .collect::<Result<HashSet<_>, _>>()?;
        Ok(Card { id, win, own })
    }
}

pub struct Day;

impl Day {
    fn points(common: u8) -> u32 {
        (1..=common)
            .fold(0, |acc, i| acc + (i * 2) as u32) - 1
    }
}

impl<'a> Solution<'a> for Day {
    type Input = Result<Vec<Card>, ParseError>;
    type Output = Option<u32>;
    const DAY: &'a str = "Day04";

    fn part1(input: &Self::Input) -> Self::Output {
        if let Ok(cards) = input {
            return Some(cards.into_iter()
                .map(|card| {
                    card.own.intersection(&card.win).count()
                })
                .filter(|&common| common > 0)
                .map(|common| 2u32.pow(common as u32 - 1)).sum::<u32>());
        }
        println!("Error: {:?}", input);
        None
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let input = input.as_ref().ok()?;
        let mut sum = 0;
        let mut repeats = vec![1u32; input.len()];
        for i in 0..input.len() {
            let matching = input[i].own.intersection(&input[i].win).count();
            sum += repeats[i];
            while repeats[i] > 0 {
                for j in 1..=matching {
                    repeats[i + j] += 1;
                }
                repeats[i] -= 1;
            }
        }
        Some(sum)
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.into_iter()
            .map(|line| Card::from_str(line))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day04::{Card, Day, ParseError};
    use crate::util::Solution;

    fn test_input() -> Vec<String> {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".lines().map(|l| l.to_string()).collect::<Vec<_>>()
    }

    #[test]
    fn test_example_part1() {
        let input = test_input();
        assert_eq!(Day::part1(&Day::parse_input(&input)), Some(13));
    }

    #[test]
    fn test_example_part2() {
        let input = test_input();
        assert_eq!(Day::part2(&Day::parse_input(&input)), Some(30));
    }
}