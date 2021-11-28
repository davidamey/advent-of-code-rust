use crate::day::Day;
use regex::Regex;
use std::io::{BufRead, BufReader, Read};

pub struct AoC;

impl Day for AoC {
    type Input = Vec<(usize, usize, char, String)>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input<R: Read>(&self, r: R) -> Self::Input {
        let re = Regex::new(r#"^(\d+)-(\d+) (.): (.+)$"#).unwrap();

        BufReader::new(r)
            .lines()
            .flatten()
            .map(|l| {
                let cs = re.captures(&l).unwrap();
                (
                    cs[1].parse().unwrap(),
                    cs[2].parse().unwrap(),
                    cs[3].chars().nth(0).unwrap(),
                    cs[4].to_string(),
                )
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|(a, b, c, d)| {
                let len = d.chars().filter(|x| x == c).count();
                len >= *a && len <= *b
            })
            .count()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .filter(|(a, b, c, d)| {
                let cs: Vec<char> = d.chars().collect();
                (cs[*a - 1] == *c || cs[*b - 1] == *c) && cs[*a - 1] != cs[*b - 1]
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::year2020::day02::*;

    #[test]
    fn test_solve_first() {
        assert_eq!(
            AoC.solve_first(&vec![
                (1, 3, 'a', "abcde".to_string()),
                (1, 3, 'b', "cdefg".to_string()),
                (2, 9, 'c', "ccccccccc".to_string())
            ]),
            2
        );
    }

    #[test]
    fn test_solve_second() {
        assert_eq!(
            AoC.solve_second(&vec![
                (1, 3, 'a', "abcde".to_string()),
                (1, 3, 'b', "cdefg".to_string()),
                (2, 9, 'c', "ccccccccc".to_string())
            ]),
            1
        );
    }
}
