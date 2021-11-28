use crate::day::Day;
use std::io::{BufRead, BufReader, Read};

pub struct AoC;

impl Day for AoC {
    type Input = Vec<Vec<char>>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|l| l.chars().collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        AoC::traverse(input, 3, 1)
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|(dx, dy)| AoC::traverse(input, *dx, *dy))
            .product()
    }
}

impl AoC {
    fn traverse(input: &<Self as Day>::Input, dx: usize, dy: usize) -> usize {
        let (mut x, mut y) = (0, 0);
        let mut trees = 0;
        while y < input.len() {
            if input[y][x] == '#' {
                trees += 1
            }
            x = (x + dx) % input[0].len();
            y += dy;
        }
        trees
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2020::day03::*;

    #[test]
    fn test_solve_first() {
        assert_eq!(
            AoC.solve_first(&vec![
                "..##.......".chars().collect(),
                "#...#...#..".chars().collect(),
                ".#....#..#.".chars().collect(),
                "..#.#...#.#".chars().collect(),
                ".#...##..#.".chars().collect(),
                "..#.##.....".chars().collect(),
                ".#.#.#....#".chars().collect(),
                ".#........#".chars().collect(),
                "#.##...#...".chars().collect(),
                "#...##....#".chars().collect(),
                ".#..#...#.#".chars().collect(),
            ]),
            7
        );
    }

    #[test]
    fn test_solve_second() {
        assert_eq!(
            AoC.solve_second(&vec![
                "..##.......".chars().collect(),
                "#...#...#..".chars().collect(),
                ".#....#..#.".chars().collect(),
                "..#.#...#.#".chars().collect(),
                ".#...##..#.".chars().collect(),
                "..#.##.....".chars().collect(),
                ".#.#.#....#".chars().collect(),
                ".#........#".chars().collect(),
                "#.##...#...".chars().collect(),
                "#...##....#".chars().collect(),
                ".#..#...#.#".chars().collect(),
            ]),
            336
        );
    }
}
