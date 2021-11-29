use crate::day::Day;
use regex::Regex;
use std::io::{BufReader, Read};

pub struct AoC;

impl Day for AoC {
    type Input = Vec<Vec<(String, String)>>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let mut data = String::new();
        BufReader::new(r)
            .read_to_string(&mut data)
            .expect("Unable to read input");

        let re = Regex::new(r"\s*(\w+):(#?\w+)\s*").unwrap();

        data.split("\n\n")
            .map(|s| {
                re.captures_iter(s)
                    .map(|cs| (cs[1].to_string(), cs[2].to_string()))
                    .filter(|(k, _)| k != "cid")
                    .collect()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        // No passports have duplicate keys so count is enough
        input.iter().filter(|&pass| pass.len() == 7).count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        let valid_eye_colours = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        input
            .iter()
            .filter(|&pass| pass.len() == 7)
            .filter(|&pass| {
                pass.iter().all(|(k, v)| match k.as_str() {
                    "byr" => {
                        let x = v.parse::<u64>().unwrap_or(0);
                        x >= 1920 && x <= 2002
                    }
                    "iyr" => {
                        let x = v.parse::<u64>().unwrap_or(0);
                        x >= 2010 && x <= 2020
                    }
                    "eyr" => {
                        let x = v.parse::<u64>().unwrap_or(0);
                        x >= 2020 && x <= 2030
                    }
                    "hgt" => {
                        let x = v[..v.len() - 2].parse::<u64>().unwrap_or(0);
                        if v.ends_with("cm") {
                            x >= 150 && x <= 193
                        } else {
                            x >= 59 && x <= 76
                        }
                    }
                    "hcl" => v.len() == 7 && v.starts_with("#"),
                    "ecl" => valid_eye_colours.contains(&v.as_str()),
                    "pid" => {
                        let x = v.parse::<u64>().unwrap_or(0);
                        v.len() == 9 && x != 0
                    }
                    _ => true,
                })
            })
            .count()
    }
}
