use std::collections::HashSet;
use std::fmt::Debug;
use crate::util::Solution;

trait Item: Debug {
    fn adjacent(&self, other: &impl Item) -> bool {
        self.borders().intersection(&other.occupy()).count() > 0
    }
    fn borders(&self) -> HashSet<Pos>;
    fn occupy(&self) -> HashSet<Pos>;
}

type XY = i32;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos(XY, XY);

#[derive(Debug)]
struct Num {
    repl: String,
    pos: Pos,
}

impl Num {
    fn val(&self) -> u32 {
        self.repl.parse::<u32>().unwrap()
    }
}

impl Item for Num {
    fn borders(&self) -> HashSet<Pos> {
        let mut borders = HashSet::new();
        for i in -1..=self.repl.len() as XY {
            borders.insert(Pos(self.pos.0 + i, self.pos.1 + 1));
            borders.insert(Pos(self.pos.0 + i, self.pos.1 - 1));
        }
        borders.insert(Pos(self.pos.0 - 1, self.pos.1));
        borders.insert(Pos(self.pos.0 + self.repl.len() as XY, self.pos.1));
        borders
    }

    fn occupy(&self) -> HashSet<Pos> {
        let mut borders = HashSet::new();
        for i in 0..self.repl.len() {
            borders.insert(Pos(self.pos.0 + i as XY, self.pos.1));
        }
        borders
    }
}

#[derive(Debug)]
struct Symbol {
    ch: char,
    pos: Pos,
}

impl Item for Symbol {
    fn borders(&self) -> HashSet<Pos> {
        vec![
            Pos(self.pos.0 - 1, self.pos.1),
            Pos(self.pos.0 + 1, self.pos.1),
            Pos(self.pos.0, self.pos.1 - 1),
            Pos(self.pos.0, self.pos.1 + 1),
            Pos(self.pos.0 + 1, self.pos.1 + 1),
            Pos(self.pos.0 - 1, self.pos.1 + 1),
            Pos(self.pos.0 + 1, self.pos.1 - 1),
            Pos(self.pos.0 - 1, self.pos.1 - 1),
        ].into_iter().collect()
    }

    fn occupy(&self) -> HashSet<Pos> {
        vec![
            Pos(self.pos.0, self.pos.1),
        ].into_iter().collect()
    }
}

#[derive(Debug)]
pub struct Schematic {
    nums: Vec<Num>,
    symbols: Vec<Symbol>,
}

pub struct Day;

impl<'a> Solution<'a> for Day {
    type Input = Schematic;
    type Output = u32;
    const DAY: &'a str = "Day03";

    fn part1(input: &Self::Input) -> Self::Output {
        input.nums.iter()
            .filter(|num| {
                input.symbols.iter()
                    .any(|symbol| num.adjacent(symbol))
            }).map(|num| num.val()).sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input.symbols.iter()
            .filter(|symbol| symbol.ch == '*')
            .map(|symbol| {
                input.nums.iter()
                    .filter(|num| num.adjacent(symbol))
                    .map(|num| num.val()).collect::<Vec<_>>()
            }).filter(|nums| nums.len() == 2)
            .map(|nums| nums[0] * nums[1])
            .sum()
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        let mut nums = Vec::new();
        let mut symbols = Vec::new();

        raw_input.into_iter().enumerate()
            .for_each(|(y, line)| {
                let mut num_buffer = String::new();
                line.chars().enumerate()
                    .for_each(|(x, ch)| {
                        match ch {
                            ch if ch.is_digit(10) => num_buffer.push(ch),
                            other => {
                                if !num_buffer.is_empty() {
                                    nums.push(Num {
                                        repl: num_buffer.clone(),
                                        pos: Pos((x - num_buffer.len()) as XY, y as XY),
                                    });
                                    num_buffer.clear();
                                }
                                match other {
                                    '.' => (),
                                    symbol => symbols.push(Symbol {
                                        ch: symbol,
                                        pos: Pos(x as XY, y as XY),
                                    })
                                }
                            }
                        }
                    });
                if !num_buffer.is_empty() {
                    nums.push(Num {
                        repl: num_buffer.clone(),
                        pos: Pos((line.len() - num_buffer.len()) as XY, y as XY),
                    });
                    num_buffer.clear();
                }
            });
        Schematic { nums, symbols }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".lines().map(|l| l.to_string()).collect::<Vec<String>>();
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), 4361);
    }

    #[test]
    fn test_example_part2() {
        let input = "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56".lines().map(|l| l.to_string()).collect::<Vec<String>>();
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), 925);
    }
}
