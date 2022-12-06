use std::io::{self, BufRead, BufReader};

use crate::day::Day;

pub struct AoC;

impl Day for AoC {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        BufReader::new(r).lines().flatten().collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|rucksack| {
                let (cmp1, cmp2) = rucksack.split_at(rucksack.len() / 2);
                let shared_item = cmp1.chars().find(|&c| cmp2.contains(c)).unwrap();
                to_priority(shared_item)
            })
            .sum()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .chunks(3)
            .map(|rucksacks| {
                let badge = rucksacks[0]
                    .chars()
                    .find(|&c| rucksacks[1].contains(c) && rucksacks[2].contains(c))
                    .unwrap();
                to_priority(badge)
            })
            .sum()
    }
}

fn to_priority(c: char) -> usize {
    if c > 'a' {
        1 + (c as usize) - ('a' as usize)
    } else {
        27 + (c as usize) - ('A' as usize)
    }
}
