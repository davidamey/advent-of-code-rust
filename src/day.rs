// inspired by https://github.com/noirotm/advent-of-code-2019/tree/master/src

use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn input_file(year: i32, day: i32) -> String {
    format!("input/{}/day{:02}", year, day)
}

#[allow(dead_code)]
pub fn read_to_vec<R: io::Read>(r: R) -> Vec<String> {
    let r = BufReader::new(r);
    r.lines().filter_map(|l| l.ok()).collect()
}

pub trait Day {
    type Input;
    type Output1: Display;
    type Output2: Display;

    fn parse_input<R: io::Seek + io::Read>(&self, r: R) -> Self::Input;
    fn solve_first(&self, input: &Self::Input) -> Self::Output1;
    fn solve_second(&self, input: &Self::Input) -> Self::Output2;

    fn load_input<P: AsRef<Path>>(&self, p: P) -> io::Result<Self::Input> {
        let f = File::open(p)?;
        Ok(self.parse_input(f))
    }

    fn solve(&self, year: i32, day: i32) {
        let input_file = input_file(year, day);
        let input = self
            .load_input(input_file)
            .expect("unable to open input file");
        let s1 = self.solve_first(&input);
        let s2 = self.solve_second(&input);
        println!("Part 1: {}", s1);
        println!("Part 2: {}", s2);
    }
}
