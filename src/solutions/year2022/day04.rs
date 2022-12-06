use regex::Regex;
use std::io::{self, BufRead, BufReader};

use crate::day::Day;

pub struct AoC;

impl Day for AoC {
    type Input = Vec<(usize, usize, usize, usize)>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let re = Regex::new(r#"^(\d+)-(\d+),(\d+)-(\d+)$"#).unwrap();

        BufReader::new(r)
            .lines()
            .flatten()
            .map(|l| {
                let cs = re.captures(&l).unwrap();
                (
                    cs[1].parse().unwrap(),
                    cs[2].parse().unwrap(),
                    cs[3].parse().unwrap(),
                    cs[4].parse().unwrap(),
                )
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|&(a, b, x, y)| (a <= x && b >= y) || (x <= a && y >= b))
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .filter(|&(a, b, x, y)| a <= y && x <= b)
            .count()
    }
}
