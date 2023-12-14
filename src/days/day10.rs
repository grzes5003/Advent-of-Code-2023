use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use crate::err::InputError;
use crate::util::Solution;


type Coord = (isize, isize);

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Debug)]
enum Pipe {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Start,
    None,
}

impl FromStr for Pipe {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Pipe::Horizontal),
            "|" => Ok(Pipe::Vertical),
            "J" => Ok(Pipe::TopLeft),
            "L" => Ok(Pipe::TopRight),
            "7" => Ok(Pipe::BottomLeft),
            "F" => Ok(Pipe::BottomRight),
            "S" => Ok(Pipe::Start),
            "." => Ok(Pipe::None),
            _ => Err(InputError::WrongFormat(format!("Invalid pipe: {}", s)))
        }
    }
}

impl Pipe {
    fn connected(&self, other: &Self, dir: &Direction) -> bool {
        match dir {
            Direction::Down => {
                match (self, other) {
                    (Pipe::Vertical | Pipe::Start | Pipe::TopLeft | Pipe::TopRight,
                        Pipe::BottomLeft | Pipe::BottomRight | Pipe::Vertical) => true,
                    _ => false
                }
            }
            Direction::Up => {
                match (self, other) {
                    (Pipe::Vertical | Pipe::Start | Pipe::BottomLeft | Pipe::BottomRight,
                        Pipe::TopLeft | Pipe::TopRight | Pipe::Vertical) => true,
                    _ => false
                }
            }
            Direction::Left => {
                match (self, other) {
                    (Pipe::Horizontal | Pipe::Start | Pipe::TopLeft | Pipe::BottomLeft,
                        Pipe::TopRight | Pipe::BottomRight | Pipe::Horizontal) => true,
                    _ => false
                }
            }
            Direction::Right => {
                match (self, other) {
                    (Pipe::Horizontal | Pipe::Start | Pipe::TopRight | Pipe::BottomRight,
                        Pipe::TopLeft | Pipe::BottomLeft | Pipe::Horizontal) => true,
                    _ => false
                }
            }
        }
    }
}


pub struct Maze {
    map: HashMap<Coord, Pipe>,
    start: Coord,
}

impl Debug for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self.size();
        writeln!(f, " :{}", (0..=max_x)
            .map(|i| ((i % 10) as u8 + 48) as char)
            .collect::<String>())?;
        for y in 0..=max_y {
            write!(f, "{}:", y % 10)?;
            for x in 0..=max_x {
                if let Some(pipe) = self.map.get(&(x, y)) {
                    match pipe {
                        Pipe::Horizontal => write!(f, "-")?,
                        Pipe::Vertical => write!(f, "|")?,
                        Pipe::TopLeft => write!(f, "J")?,
                        Pipe::TopRight => write!(f, "L")?,
                        Pipe::BottomLeft => write!(f, "7")?,
                        Pipe::BottomRight => write!(f, "F")?,
                        Pipe::Start => write!(f, "S")?,
                        Pipe::None => write!(f, ".")?,
                    }
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Maze {
    fn neighbours(&self, coord: Coord) -> Vec<Coord> {
        let (x, y) = coord;
        if let Some(pipe) = self.map.get(&(x, y)) {
            return vec![
                ((coord.0, coord.1 + 1), Direction::Up),
                ((coord.0, coord.1 - 1), Direction::Down),
                ((coord.0 - 1, coord.1), Direction::Left),
                ((coord.0 + 1, coord.1), Direction::Right),
            ]
                .into_iter()
                .filter(|(coord, dir)| {
                    if let Some(other) = self.map.get(coord) {
                        pipe.connected(other, dir)
                    } else {
                        false
                    }
                })
                .map(|(coord, _)| coord)
                .collect();
        }
        Vec::new()
    }

    fn cycle(&self, start: Coord) -> Vec<Coord> {
        let mut queue = VecDeque::from(vec![(start, (-1, -1))]);
        let mut visited = vec![start];
        while let Some((node, _)) = queue.pop_front() {
            for neighbour in self.neighbours(node) {
                if visited.contains(&neighbour) && Maze::already_seen(&queue, (neighbour, node)) {
                    return visited;
                }
                if !visited.contains(&neighbour) {
                    visited.push(neighbour);
                    queue.push_back((neighbour, node));
                }
            }
        }
        visited
    }

    fn already_seen(input: &VecDeque<(Coord, Coord)>, item: (Coord, Coord)) -> bool {
        for (child, parent) in input {
            if child == &item.0 && parent != &item.1 {
                return true;
            }
        }
        false
    }

    fn area(&self, input: &Vec<Coord>) -> usize {
        let corners = vec![
            Pipe::Start,
            Pipe::TopLeft,
            Pipe::TopRight,
            Pipe::BottomLeft,
            Pipe::BottomRight,
        ];

        let mut path = self.path(input).into_iter()
            .filter(|coord| corners.contains(self.map.get(coord).unwrap()))
            .collect::<Vec<Coord>>();

        path.push(path[0]);

        let a = path.windows(2)
            .map(|window| {
                let (x1, y1) = window[0];
                let (x2, y2) = window[1];
                let x = (x1 * y2) - (y1 * x2);
                x
            }).sum::<isize>().abs() as usize / 2;
        a - (input.len() / 2) + 1
    }

    fn path(&self, input: &Vec<Coord>) -> Vec<Coord> {
        let mut path = vec![input[0]];
        let mut seen = vec![input[0]];
        while seen.len() != input.len() {
            for coord in input {
                if seen.contains(coord) {
                    continue;
                }
                if let Some(last) = path.last() {
                    if self.neighbours(*last).contains(coord) {
                        path.push(*coord);
                        seen.push(*coord);
                    }
                }
            }
        }
        path
    }

    fn size(&self) -> Coord {
        let mut max_x = 0;
        let mut max_y = 0;
        for (x, y) in self.map.keys() {
            if *x > max_x {
                max_x = *x;
            }
            if *y > max_y {
                max_y = *y;
            }
        }
        (max_x, max_y)
    }
}

pub struct Day;

impl<'a> Solution<'a> for Day {
    type Input = Maze;
    type Output = usize;
    const DAY: &'a str = "Day10";

    fn part1(input: &Self::Input) -> Self::Output {
        let path = input.cycle(input.start);
        path.len() / 2
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let path = input.cycle(input.start);
        input.area(&path)
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        let mut start = (0, 0);
        let mut map = HashMap::new();
        for (y, line) in raw_input.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = (x as isize, y as isize);
                let pipe = Pipe::from_str(&c.to_string()).unwrap();
                if pipe == Pipe::Start {
                    start = coord;
                }
                if pipe != Pipe::None {
                    map.insert(coord, pipe);
                }
            }
        }
        Maze { map, start }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...".lines().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        let input = &Day::parse_input(&get_input());
        println!("{:?}", input);
        assert_eq!(Day::part1(input), 4);
    }
}