use itertools::Itertools;
use std::io::{self, BufRead, BufReader};

use crate::day::Day;

pub struct AoC;

impl Day for AoC {
    type Input = Vec<char>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .chars()
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        find_marker(input, 4)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        find_marker(input, 14)
    }
}

fn find_marker(input: &Vec<char>, size: usize) -> usize {
    input
        .windows(size)
        .enumerate()
        .find(|&(_, w)| w.len() == w.iter().sorted().dedup().collect_vec().len())
        .unwrap()
        .0
        + size
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2022::day06::*;

    #[test]
    fn test_solve_first() {
        assert_eq!(
            AoC.solve_first(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect()),
            7
        );
    }

    #[test]
    fn test_solve_second() {
        assert_eq!(
            AoC.solve_second(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect()),
            19
        );
    }
}
