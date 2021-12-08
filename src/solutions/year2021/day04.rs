use crate::day::Day;
use std::io::{self, BufRead, BufReader};

pub struct AoC;

impl Day for AoC {
    type Input = (Vec<u64>, Vec<Board>);
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let lines: Vec<String> = BufReader::new(r).lines().flatten().collect();
        let counts: Vec<u64> = lines[0].split(",").map(|c| c.parse().unwrap()).collect();
        let boards: Vec<Board> = lines[1..]
            .chunks(6)
            .map(|c| Board {
                finished: false,
                matched: vec![false; 25],
                values: c
                    .iter()
                    .skip(1)
                    .flat_map(|l| {
                        l.split_ascii_whitespace()
                            .map(|i| i.parse().unwrap())
                            .collect::<Vec<u64>>()
                    })
                    .collect(),
            })
            .collect();
        (counts, boards)
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        let (calls, mut boards) = input.clone();
        for c in calls {
            for b in &mut boards {
                if b.is_winner_with(c) {
                    return b.sum_unmarked() * c;
                }
            }
        }
        panic!("No board won")
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output1 {
        let (calls, mut boards) = input.clone();
        let mut unfinished = boards.len();
        for c in calls {
            for b in &mut boards {
                if b.finished {
                    continue;
                }

                if b.is_winner_with(c) {
                    b.finished = true;
                    unfinished -= 1;

                    if unfinished == 0 {
                        return b.sum_unmarked() * c;
                    }
                }
            }
        }
        panic!("No board won")
    }
}

#[derive(Clone)]
pub struct Board {
    values: Vec<u64>,
    matched: Vec<bool>,
    finished: bool,
}

impl Board {
    fn is_winner_with(&mut self, call: u64) -> bool {
        for i in 0..25 {
            if self.values[i] == call {
                self.matched[i] = true;
                return self.is_winner(i);
            }
        }
        false
    }

    fn is_winner(&self, i: usize) -> bool {
        let row = (0..5).all(|x| self.matched[i / 5 * 5 + x]);
        let col = (0..5).all(|x| self.matched[(i + x * 5) % 25]);
        row || col
    }

    fn sum_unmarked(&self) -> u64 {
        (0..25).fold(0, |acc, i| {
            if !self.matched[i] {
                acc + self.values[i]
            } else {
                acc
            }
        })
    }
}
