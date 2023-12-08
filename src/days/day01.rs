use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::util::Solution;


type Num = u32;

pub struct Day;


lazy_static! {
    static ref MAP: HashMap<&'static str, u8> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ].into_iter().collect();
}

impl Day {
    fn despell(inp: String) -> String {
        MAP.iter()
            .map(|(word, num)| (inp.find(word), word, num))
            .filter(|(pos, _, _)| pos.is_some())
            .min_by(|a, b| a.0.cmp(&b.0))
            .and_then(|(pos, word, num)| {
                let mut inp = inp.clone();
                inp.replace_range(pos?..pos? + word.len(), &num.to_string());
                return Some(inp)
            }).unwrap_or(inp)
    }

    fn rdespell(inp: String) -> String {
        MAP.iter()
            .map(|(word, num)| (inp.rfind(word), word, num))
            .max_by(|a, b| a.0.cmp(&b.0))
            .and_then(|(pos, word, num)| {
                let mut inp = inp.clone();
                inp.replace_range(pos?..pos? + word.len(), &num.to_string());
                return Some(inp)
            }).unwrap_or(inp)
    }
}

trait Rev {
    fn rev(&self) -> Self;
}

impl Rev for String {
    fn rev(&self) -> Self {
        self.chars().rev().collect::<String>()
    }
}

impl<'a> Solution<'a> for Day {
    type Input = Vec<String>;
    type Output = Option<Num>;
    const DAY: &'a str = "Day01";

    fn part1(input: &Self::Input) -> Self::Output {
        input.into_iter()
            .map(|line|
                (line.chars().find(|ch| ch.is_digit(10)),
                 line.chars().rfind(|ch| ch.is_digit(10))))
            .map(|(t1, t2)|
                (t1.and_then(|i| i.to_digit(10)),
                 t2.and_then(|i| i.to_digit(10))))
            .fold(Some(0u32), |acc, (a, b)| Some(a? * 10 + b? + acc?))
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input.into_iter()
            .map(|line|
                (Day::despell(line.to_owned()),
                 Day::rdespell(line.to_owned())))
            .map(|(a, b)|
                (a.chars().find(|ch| ch.is_digit(10)),
                 b.chars().rfind(|ch| ch.is_digit(10))))
            .map(|(t1, t2)|
                (t1.and_then(|i| i.to_digit(10)),
                 t2.and_then(|i| i.to_digit(10))))
            .fold(Some(0u32), |acc, (a, b)| Some(a? * 10 + b? + acc?))
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use crate::bench_day;
    use crate::util::parse;

    #[test]
    fn test_example_t1() {
        let input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), Some(142));
    }

    #[test]
    fn test_example_t2() {
        let input = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        let input = Day::parse_input(&input);
        assert_eq!(Day::part2(&input), Some(281));
    }

    bench_day!(Day);
}