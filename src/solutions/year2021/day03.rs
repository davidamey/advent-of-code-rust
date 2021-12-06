use crate::day::Day;
use std::io::{self, BufRead, BufReader};

pub struct AoC;

impl Day for AoC {
    type Input = Vec<Vec<char>>;
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r)
            .lines()
            .flatten()
            .map(|l| l.chars().collect())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (mut g, mut e) = (0, 0);
        for i in 0..input[0].len() {
            let (zeros, ones) = counts(input, i);
            if ones > zeros {
                g = (g << 1) + 1;
                e <<= 1;
            } else {
                g <<= 1;
                e = (e << 1) + 1;
            }
        }
        g * e
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output1 {
        let og = rating(input, |zeros, ones| if ones >= zeros { '1' } else { '0' });
        let co = rating(input, |zeros, ones| if ones < zeros { '1' } else { '0' });
        og * co
    }
}

fn rating(input: &Vec<Vec<char>>, f: fn(usize, usize) -> char) -> u64 {
    (0..input[0].len()).fold(input.clone(), |data, pos| {
        if data.len() == 1 {
            data
        } else {
            let (zeros, ones) = counts(&data, pos);
            let x = f(zeros, ones);
            data.into_iter().filter(|d| d[pos] == x).collect()
        }
    })[0]
        .iter()
        .fold(
            0,
            |acc, &x| if x == '1' { (acc << 1) + 1 } else { acc << 1 },
        )
}

fn counts(data: &Vec<Vec<char>>, pos: usize) -> (usize, usize) {
    data.iter().fold((0, 0), |(a, b), d| match d[pos] {
        '0' => (a + 1, b),
        '1' => (a, b + 1),
        _ => panic!(),
    })
}
