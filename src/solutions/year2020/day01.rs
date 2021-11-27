use crate::day::Day;
use itertools::Itertools;
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
            .flat_map(|l| l.parse())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .tuple_combinations()
            .find(|&(a, b)| a + b == 2020)
            .map(|(a, b)| a * b)
            .unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .tuple_combinations()
            .find(|&(a, b, c)| a + b + c == 2020)
            .map(|(a, b, c)| a * b * c)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2020::day01::*;

    #[test]
    fn test_solve_first() {
        assert_eq!(
            AoC.solve_first(&vec![1721, 979, 366, 299, 675, 1456]),
            514579
        );
    }

    #[test]
    fn test_solve_second() {
        assert_eq!(
            AoC.solve_second(&vec![1721, 979, 366, 299, 675, 1456]),
            241861950
        );
    }
}
