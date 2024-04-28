use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use crate::util::Solution;

type Pos = (isize, isize);


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    X
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
pub enum Tile {
    Empty,      //# .
    Horizontal, //# -
    Vertical,   //# |
    FMirror,    //# /
    TMirror,    //# \
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
struct Bean {
    pos: Pos,
    dir: Direction,
}

impl Bean {
    fn next_pos(&self) -> Pos {
        let (x, y) = self.pos;
        match self.dir {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::X => (x, y),
        }
    }
}

pub struct Cave {
    tiles: Box<[Box<[Tile]>]>,
    beans: Vec<Bean>,
    visited: HashSet<Bean>,
}

impl Debug for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = (self.tiles[0].len(), self.tiles.len());
        let mut cave = Cave::from(&self.tiles);
        while cave.step() {}
        let hs = cave.visited.into_iter()
            .map(|bean| bean.pos)
            .collect::<HashSet<Pos>>();
        for y in 0..max_y {
            for x in 0..max_x {
                if hs.contains(&(x as isize, y as isize)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Cave {
    fn step(&mut self) -> bool {
        let mut beans_to_add = Vec::new();
        let mut beans_to_remove = Vec::new();
        let mut counter = 0;
        for bean in self.beans.iter_mut() {
            let (x, y) = bean.next_pos();
            if x < 0 || y < 0
                || x >= self.tiles[0].len() as isize
                || y >= self.tiles.len() as isize {
                beans_to_remove.push(bean.clone());
                counter += 1;
                continue;
            }
            bean.pos = (x, y);
            if self.visited.contains(bean) {
                println!("visited {}: {:?}", counter, bean);
                beans_to_remove.push(bean.clone());
                counter += 1;
                continue;
            }
            match self.tiles[y as usize][x as usize] {
                Tile::FMirror => {
                    bean.dir = match bean.dir {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                        Direction::X => Direction::X,
                    }
                }
                Tile::TMirror => {
                    bean.dir = match bean.dir {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                        Direction::X => Direction::X,
                    }
                }
                Tile::Vertical => {
                    bean.dir = match bean.dir {
                        Direction::Up => Direction::Up,
                        Direction::Down => Direction::Down,
                        Direction::Left | Direction::Right => {
                            beans_to_add.push(Bean {
                                pos: (x, y),
                                dir: Direction::Up,
                            });
                            Direction::Down
                        },
                        Direction::X => Direction::X,
                    }
                }
                Tile::Horizontal => {
                    bean.dir = match bean.dir {
                        Direction::Left => Direction::Left,
                        Direction::Right => Direction::Right,
                        Direction::Up | Direction::Down => {
                            beans_to_add.push(Bean {
                                pos: (x, y),
                                dir: Direction::Left,
                            });
                            Direction::Right
                        },
                        Direction::X => Direction::X,
                    }
                }
                Tile::Empty => {}
            }
        }
        for bean in beans_to_remove {
            self.beans.iter()
                .position(|b| *b == bean)
                .map(|pos| self.beans.remove(pos));
        }

        let visited_len = self.visited.len();
        self.visited.extend(beans_to_add.iter().map(|bean| Bean {
            pos: bean.pos,
            dir: Direction::X,
        }));
        self.visited.extend(self.beans.iter());
        self.beans.append(&mut beans_to_add);
        println!("{}/{}; {}->{}", counter, self.beans.len(), visited_len, self.visited.len());
        visited_len != self.visited.len()
    }
}

impl From<&Box<[Box<[Tile]>]>> for Cave {
    fn from(value: &Box<[Box<[Tile]>]>) -> Self {
        Cave {
            tiles: value.iter().map(|item|
                item.deref().to_vec().into_boxed_slice()
            ).collect::<Vec<_>>().into_boxed_slice(),
            beans: vec![Bean {
                pos: (-1, 0),
                dir: Direction::Right,
            }],
            visited: HashSet::new(),
        }
    }
}

pub struct Day;

impl<'a> Solution<'a> for Day {
    type Input = Box<[Box<[Tile]>]>;
    type Output = usize;
    const DAY: &'a str = "Day16";

    fn part1(input: &Self::Input) -> Self::Output {
        let mut cave = Cave::from(input);
        println!("{:?}", cave);
        while cave.step() {
            // println!("====================");
        }
        cave.visited.into_iter()
            .map(|bean| bean.pos)
            .collect::<HashSet<Pos>>()
            .len()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        1
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        let mut tiles = Vec::new();
        for line in raw_input {
            let mut row = Vec::new();
            for ch in line.chars() {
                match ch {
                    '.' => row.push(Tile::Empty),
                    '-' => row.push(Tile::Horizontal),
                    '|' => row.push(Tile::Vertical),
                    '/' => row.push(Tile::FMirror),
                    '\\' => row.push(Tile::TMirror),
                    _ => {}
                }
            }
            tiles.push(row.into_boxed_slice());
        }
        tiles.into_boxed_slice()
    }
}