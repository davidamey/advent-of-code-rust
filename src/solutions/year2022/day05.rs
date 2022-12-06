use itertools::Itertools;
use regex::Regex;
use std::io::{self, BufRead, BufReader};

use crate::day::Day;

pub struct AoC;

impl Day for AoC {
    type Input = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);
    type Output1 = String;
    type Output2 = String;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let lines: Vec<String> = BufReader::new(r).lines().flatten().collect();

        let (setup_raw, moves_raw) = lines.split(|l| l.is_empty()).next_tuple().unwrap();

        let setup: Vec<Vec<char>> = (1..setup_raw.last().unwrap().len())
            .step_by(4)
            .map(|stack| {
                setup_raw
                    .iter()
                    .rev()
                    .skip(1) // the indexes
                    .filter_map(|l| {
                        let c = l.as_bytes()[stack] as char;
                        if c == ' ' {
                            None
                        } else {
                            Some(c)
                        }
                    })
                    .collect()
            })
            .collect();

        let re = Regex::new(r#"^move (\d+) from (\d+) to (\d+)$"#).unwrap();
        let moves: Vec<(usize, usize, usize)> = moves_raw
            .iter()
            .map(|m| {
                let cs = re.captures(&m).unwrap();
                (
                    cs[1].parse().unwrap(),
                    cs[2].parse().unwrap(),
                    cs[3].parse().unwrap(),
                )
            })
            .collect();

        (setup, moves)
    }

    fn solve_first(&self, (setup, moves): &Self::Input) -> Self::Output1 {
        let mut stacks: Vec<Vec<char>> = setup.clone();
        for &(n, from, to) in moves {
            for _ in 0..n {
                let c = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(c)
            }
        }
        stacks.iter().map(|s| s.last().unwrap()).collect()
    }

    fn solve_second(&self, (setup, moves): &Self::Input) -> Self::Output2 {
        let mut stacks: Vec<Vec<char>> = setup.clone();
        for &(n, from, to) in moves {
            let i = stacks[from - 1].len() - n;
            let mut lifted = stacks[from - 1].split_off(i);
            stacks[to - 1].append(&mut lifted);
        }
        stacks.iter().map(|s| s.last().unwrap()).collect()
    }
}
