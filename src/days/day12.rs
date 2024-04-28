use std::str::FromStr;
use crate::util::Solution;


pub struct Springs {
    statuses: Vec<Option<bool>>,
    groups: Vec<u16>,
}

impl Springs {
    fn variants(springs: Springs, vals: usize) -> usize {
        let statuses = springs.statuses;
        let groups = springs.groups;
        if statuses.len() == 0 && groups.len() == 0 {
            return 1;
        }
        if statuses.len() == 0 || groups.len() == 0 {
            return 0;
        }
        let group = groups[0];
        let mut add = 0;
        if statuses.len() > group as usize && statuses.iter().take(group as usize).all(|s| s.is_none() || *s == Some(false))
            && statuses.get(group as usize + 1) != Some(&Some(false))
        {
            let new_len = std::cmp::min(statuses.len(), group as usize + 1);
            add += Springs::variants(
                Springs {
                    statuses: statuses[new_len..].to_vec(),
                    groups: groups[1..].to_vec(),
                }, vals);
        }

        add += Springs::variants(
            Springs {
                statuses: statuses[1..].to_vec(),
                groups: groups.clone(),
            }, vals);

        add + vals
    }
}

impl FromStr for Springs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let [statuses, groups] = s.split_whitespace().collect::<Vec<&str>>()[..] {
            let statuses = statuses.chars().map(|c| match c {
                '.' => Some(true),
                '#' => Some(false),
                _ => None
            }).collect::<Vec<Option<bool>>>();
            let groups = groups.split(',')
                .map(|val| val.parse()
                    .map_err(|_| ())).collect::<Result<Vec<u16>, Self::Err>>()?;
            return Ok(Springs { statuses, groups });
        }
        Err(())
    }
}

pub struct Day;


impl<'a> Solution<'a> for Day {
    type Input = Vec<Springs>;
    type Output = ();
    const DAY: &'a str = "Day12";

    fn part1(input: &Self::Input) -> Self::Output {
        input.into_iter()
            .map(|s| Springs {
                statuses: s.statuses.clone(),
                groups: s.groups.clone(),
            })
            .map(|springs| Springs::variants(springs, 0))
            .for_each(|v| println!("{}", v));
    }

    fn part2(input: &Self::Input) -> Self::Output {
        ()
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.iter()
            .map(|line| line.parse().unwrap())
            .collect()
    }
}
