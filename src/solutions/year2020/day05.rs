use itertools::Itertools;

use crate::day::Day;
use std::io::{BufRead, BufReader, Read};

pub struct AoC;

impl Day for AoC {
    type Input = Vec<u64>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|s| 8 * AoC::decode(&s[..7]) + AoC::decode(&s[7..]))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        *input.iter().max().unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        *input
            .iter()
            .sorted()
            .tuple_windows()
            .find(|&(a, b)| (a + 1) != *b)
            .unwrap()
            .0
            + 1
    }
}

impl AoC {
    fn decode(s: &str) -> u64 {
        s.chars().fold(0, |acc, x| match x {
            'F' | 'L' => acc << 1,
            'B' | 'R' => (acc << 1) + 1,
            _ => panic!("Invalid seat"),
        })
    }
}
