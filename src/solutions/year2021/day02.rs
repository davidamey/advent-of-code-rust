use crate::day::Day;
use std::io::{self, BufRead, BufReader};

pub struct AoC;

impl Day for AoC {
    type Input = Vec<(String, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|l| {
                let parts: Vec<&str> = l.split_whitespace().collect();
                (parts[0].to_string(), parts[1].parse().unwrap())
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (h, d) = input
            .iter()
            .fold((0, 0), |(h, d), (dir, v)| match dir.as_str() {
                "forward" => (h + v, d),
                "down" => (h, d + v),
                "up" => (h, d - v),
                _ => panic!("unknown dir"),
            });
        h * d
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output1 {
        let (h, d, _) = input
            .iter()
            .fold((0, 0, 0), |(h, d, a), (dir, v)| match dir.as_str() {
                "forward" => (h + v, d + (a * v), a),
                "down" => (h, d, a + v),
                "up" => (h, d, a - v),
                _ => panic!("unknown dir"),
            });
        h * d
    }
}
