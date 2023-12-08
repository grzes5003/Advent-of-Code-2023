use std::io;
use std::fs::File;
use std::path::Path;
use std::io::BufRead;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn parse(path: &str) -> Vec<String> {
    match read_lines(path) {
        Ok(lines) =>
            lines.map(|res| res.unwrap()).into_iter().collect::<Vec<_>>(),
        Err(err) => panic!("Cannot read file: {}", err.to_string())
    }
}

pub trait Solution<'a> {
    type Input;
    type Output;
    const DAY: &'a str;

    fn part1(input: &Self::Input) -> Self::Output;
    fn part2(input: &Self::Input) -> Self::Output;
    fn parse_input(raw_input: &Vec<String>) -> Self::Input;
}

pub fn solve<'a, S: Solution<'a>>() -> (S::Output, S::Output) {
    let raw_input = parse(format!("input/{}.in", S::DAY).as_str());
    let input = S::parse_input(&raw_input);
    (S::part1(&input), S::part2(&input))
}

#[macro_export]
macro_rules! bench_day {
    ($day:ident) => {
        #[bench]
        fn bench_code1(b: &mut Bencher) {
            let input = parse(format!("input/{}.in", $day::DAY).as_str());
            let input = $day::parse_input(&input);
            b.iter(|| $day::part1(&input));
        }

        #[bench]
        fn bench_code2(b: &mut Bencher) {
            let input = parse(format!("input/{}.in", $day::DAY).as_str());
            let input = $day::parse_input(&input);
            b.iter(|| $day::part2(&input));
        }
    }
}