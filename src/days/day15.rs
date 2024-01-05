use std::fmt::{Debug, Formatter};
use itertools::Itertools;
use crate::util::Solution;

pub struct Day;


trait Hashable<'a, T> {
    fn hash(&self) -> T;
}

impl<'a> Hashable<'a, u8> for &'a str {
    fn hash(&self) -> u8 {
        let mut hash = 0;
        for c in self.chars() {
            hash += c as u16;
            hash *= 17;
            hash = hash.rem_euclid(256u16);
        }
        hash as u8
    }
}

#[derive(Debug)]
enum Instruction<'a> {
    Dash(&'a str),
    Equals(&'a str, u8),
}

impl<'a> From<&'a str> for Instruction<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            val if val.ends_with('-') => Instruction::Dash(val.strip_suffix('-').unwrap()),
            val if val.contains('=') => {
                let mut split = val.split('=');
                let key = split.next().unwrap();
                let val = split.next().unwrap();
                Instruction::Equals(key, val.parse().unwrap())
            }
            _ => panic!("Invalid instruction: {}", value)
        }

    }
}

impl Hashable<'_, u8> for Instruction<'_> {
    fn hash(&self) -> u8 {
        match self {
            Instruction::Dash(s) => s.hash(),
            Instruction::Equals(s, _) => s.hash(),
        }
    }
}

struct HM<'a> {
    map: [Vec<Instruction<'a>>; 256],
}

impl<'a> HM<'a> {
    fn new() -> Self {
        let map: [Vec<Instruction<'a>>; 256] = [(); 256].map(|_| Vec::new());
        HM { map }
    }

    fn handle(&mut self, val: Instruction<'a>) {
        match val {
            Instruction::Equals(s, val) => {
                let hash = s.hash() as usize;
                let option = self.map[hash].iter().find_position(|i| match i {
                    Instruction::Equals(key, _) => key == &s,
                    _ => false
                });
                match option {
                    Some((pos, _)) => {
                        self.map[hash][pos] = Instruction::Equals(s, val);
                    }
                    None => {
                        self.map[hash].push(Instruction::Equals(s, val));
                    }
                }
            },
            Instruction::Dash(s) => {
                let hash = s.hash() as usize;
                let option = self.map[hash].iter().find_position(|i| match i {
                    Instruction::Equals(key, _) => key == &s,
                    _ => false
                });
                if let Some((pos, _)) = option {
                    self.map[hash].remove(pos);
                }
            }
        }
    }
}

impl Debug for HM<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, v) in self.map.iter().enumerate() {
            if v.len() > 0 {
                writeln!(f, "{}: {:?}", idx, v)?;
            }
        }
        Ok(())
    }
}

impl Hashable<'_, usize> for HM<'_> {
    fn hash(&self) -> usize {
        self.map.iter().enumerate()
            .map(|(box_idx, v)| v.iter().enumerate()
                .map(|(idx, i)| match i {
                    Instruction::Equals(_, val) => (box_idx + 1) * (idx + 1) * (*val as usize),
                    _ => 0,
                }).sum::<usize>())
            .sum::<usize>()
    }
}


impl<'a> Solution<'a> for Day {
    type Input = Box<[Box<str>]>;
    type Output = u32;
    const DAY: &'a str = "Day15";

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter()
            .map(|s| s.as_ref())
            .map(|a| a.hash() as u32)
            .sum::<u32>()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut hm = HM::new();
        input.iter()
            .map(|s| Instruction::from(s.as_ref()))
            .for_each(|a| hm.handle(a));
        hm.hash() as u32
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input[0].split(',')
            .map(|s| s.to_string())
            .map(|s| s.into_boxed_str())
            .collect::<Box<[Box<str>]>>()
    }
}