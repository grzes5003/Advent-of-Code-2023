use crate::util::Solution;


type Num = u64;

#[derive(Debug)]
pub struct Race {
    time: Num,
    distance: Num,
}

impl Race {
    // t_r*v = s
    // v = t_b
    // t_r = t - t_b

    // s = t_b * (t - t_b)
    // 0 < -t_b^2 + t_b * t - s

    fn zeros(&self) -> (Num, Num) {
        let sqrt_delta = ((self.time).pow(2) as f64 - (4 * self.distance) as f64).sqrt();
        let lower = 0.5f64 * (self.time as f64 - sqrt_delta);
        let upper = 0.5f64 * (self.time as f64 + sqrt_delta);
        (if lower.ceil() == lower { (lower + 1f64).floor() } else { lower.ceil() } as Num,
         if upper.floor() == upper { (upper - 1f64).ceil() } else { upper.floor() } as Num)
    }
}

pub struct Day;

impl<'a> Solution<'a> for Day {
    type Input = Vec<Race>;
    type Output = Num;
    const DAY: &'a str = "Day06";

    fn part1(input: &Self::Input) -> Self::Output {
        println!("{:?}", input);
        input.iter()
            .map(|r| r.zeros())
            .map(|(l, u)| u - l + 1)
            .product::<Num>()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let (time, dist) = input.into_iter()
            .fold(("".to_string(), "".to_string()), |acc, r| {
                (format!("{}{}", acc.0, r.time), format!("{}{}", acc.1, r.distance))
            });

        let race = Race {
            time: time.parse().unwrap(),
            distance: dist.parse().unwrap(),
        };

        println!("{:?}", race);

        let (l, u) = race.zeros();
        u - l + 1
    }

    fn parse_input(raw_input: &Vec<String>) -> Self::Input {
        raw_input[0].split_whitespace()
            .zip(raw_input[1].split_whitespace())
            .skip(1)
            .map(|(t, std)| {
                Race {
                    time: t.parse().unwrap(),
                    distance: std.parse().unwrap(),
                }
            }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        vec![
            "Time:      7  15   30".to_string(),
            "Distance:  9  40  200".to_string(),
        ]
    }

    #[test]
    fn test_example_t1() {
        let input = get_input();
        let input = Day::parse_input(&input);
        assert_eq!(Day::part1(&input), 288);
    }

    #[test]
    fn test_example_t2() {
        let input = get_input();
        let input = Day::parse_input(&input);
        assert_eq!(Day::part2(&input), 71503);
    }
}