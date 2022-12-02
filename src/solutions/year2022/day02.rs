use std::io::{self, BufRead, BufReader};

use crate::day::Day;

pub struct AoC;

impl Day for AoC {
    type Input = Vec<(isize, isize)>;
    type Output1 = isize;
    type Output2 = isize;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        fn to_score(c: char) -> isize {
            match c {
                'A' | 'X' => 0,
                'B' | 'Y' => 1,
                'C' | 'Z' => 2,
                _ => panic!("invalid move"),
            }
        }

        BufReader::new(r)
            .lines()
            .flatten()
            .map(|l| {
                let chars: Vec<char> = l.chars().collect();
                (to_score(chars[0]), to_score(chars[2]))
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|&(them, you)| {
                let result = (you - them + 4) % 3;
                you + 1 + 3 * result
            })
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .map(|&(them, result)| {
                let you = (them + result + 2) % 3;
                you + 1 + 3 * result
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2022::day02::*;

    #[test]
    fn test_solve_first() {
        assert_eq!(AoC.solve_first(&vec![(0, 1), (1, 0), (2, 2)]), 15);
    }

    #[test]
    fn test_solve_second() {
        assert_eq!(AoC.solve_second(&vec![(0, 1), (1, 0), (2, 2)]), 12);
    }
}
