use crate::util::Solution;


type Num = i32;

pub struct Day;

impl Day {
    fn interpolate(vec: Vec<Num>) -> Vec<Num> {
        let mut last = vec![vec.last().unwrap().to_owned()];
        let mut intermediate = vec.clone();
        while intermediate.iter().any(|&x| x != 0) {
            intermediate = intermediate.windows(2).into_iter()
                .map(|w| w[1] - w[0])
                .collect::<Vec<Num>>();
            last.push(intermediate.last().unwrap().to_owned());
        }
        let mut vec = vec.clone();
        vec.push(last.iter().sum::<Num>());
        vec
    }
}

impl<'a> Solution<'a> for Day {
    type Input = Vec<Vec<Num>>;
    type Output = Num;
    const DAY: &'a str = "Day09";

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter()
            .map(|line| Day::interpolate(line.to_owned()))
            .map(|vec| vec.last().unwrap().to_owned())
            .sum()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input.iter()
            .map(|line| line.iter().rev().map(|item| *item).collect())
            .map(|line| Day::interpolate(line))
            .map(|vec| vec.last().unwrap().to_owned())
            .sum()
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input.into_iter()
            .map(|line|
                line.split_whitespace()
                    .map(|num| num.parse::<Num>().unwrap())
                    .collect::<Vec<Num>>()
            ).collect::<Vec<Vec<Num>>>()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".lines().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part1() {
        let input = Day::parse_input(&get_input());
        assert_eq!(Day::part1(&input), 114);
    }

}