use itertools::Itertools;
use std::io::{self, BufReader, Read};

use crate::day::Day;

pub struct AoC;

impl Day for AoC {
    type Input = Vec<u64>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut data = String::new();
        BufReader::new(r)
            .read_to_string(&mut data)
            .expect("Unable to read input");

        data.split("\n\n")
            .into_iter()
            .map(|s: &str| {
                s.split("\n")
                    .map(|ss: &str| ss.parse::<u64>().unwrap())
                    .sum()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        *input.iter().max().unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input.iter().sorted().rev().take(3).sum()
    }
}
