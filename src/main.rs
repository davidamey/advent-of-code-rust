use crate::solutions::exec;
use std::env;

mod day;
mod solutions;

fn main() {
    let year = env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("2020"))
        .parse()
        .unwrap();

    let day = env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("1"))
        .parse()
        .unwrap();

    exec(year, day)
}
