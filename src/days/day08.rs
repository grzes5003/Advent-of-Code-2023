use std::collections::HashMap;
use std::str::FromStr;
use regex::Regex;
use crate::commons;
use crate::util::Solution;


#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(())
        }
    }
}


pub struct Desert {
    instructions: Vec<Direction>,
    map: HashMap<String, (String, String)>,
}

impl Desert {

    fn count_steps(&self, start: &str, end_cond: &dyn Fn(&str) -> bool) -> usize {
        let mut instructions = self.instructions.iter().cycle();
        let mut steps = 0;
        let mut pos = start;
        loop {
            let dir = instructions.next().unwrap();
            let (left, right) = self.map.get(pos).unwrap();
            pos = match dir {
                Direction::Left => left,
                Direction::Right => right
            };
            steps += 1;
            if end_cond(pos) {
                return steps;
            }
        }
    }

}


pub struct Day;

const START: &'static str = "AAA";
const END: &'static str = "ZZZ";


impl<'a> Solution<'a> for Day {
    type Input = Desert;
    type Output = usize;
    const DAY: &'a str = "Day08";

    fn part1(input: &Self::Input) -> Self::Output {
        let end_cond = |s: &str| s == END;
        input.count_steps(START, &end_cond)
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let end_cond = |s: &str| s.chars().last() == Some('Z');
        let steps = input.map.iter()
            .filter(|(k, _)| k.chars().last() == Some('A'))
            .map(|(k, _)| input.count_steps(k, &end_cond))
            .collect::<Vec<_>>();
        commons::lcm(steps)
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        let mut input = raw_input.iter();
        let directions = input.next().unwrap()
            .chars()
            .map(|c| c.to_string().parse())
            .collect::<Result<Vec<Direction>, _>>().unwrap();

        let re = Regex::new(r"^(.{3})\s=\s\((.{3}),\s(.{3})\)$").unwrap();
        let map = input.skip(1).map(|s| {
            let caps = re.captures(s).unwrap();
            let key = caps.get(1).unwrap().as_str().to_string();
            let left = caps.get(2).unwrap().as_str().to_string();
            let right = caps.get(3).unwrap().as_str().to_string();
            (key, (left, right))
        }).collect();

        Desert {
            instructions: directions,
            map
        }
    }
}