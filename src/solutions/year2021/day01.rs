use itertools::Itertools;

use crate::day::Day;
use std::io::{self, BufRead, BufReader};

pub struct AoC;

impl Day for AoC {
    type Input = Vec<u64>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .flat_map(|l| l.parse())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input.iter().tuple_windows().filter(|(a, b)| a < b).count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .zip(input.iter().skip(3))
            .filter(|(a, b)| a < b)
            .count()
    }
}
