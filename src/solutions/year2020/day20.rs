use crate::day::Day;
use std::io::Read;

pub struct AoC;

impl Day for AoC {
    type Input = ();
    type Output1 = u64;
    type Output2 = u64;

    fn parse_input<R: Read>(&self, _r: R) -> Self::Input {{}}

    fn solve_first(&self, _input: &Self::Input) -> Self::Output1 {{
        0
    }}

    fn solve_second(&self, _input: &Self::Input) -> Self::Output2 {{
        0
    }}
}