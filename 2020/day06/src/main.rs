#![feature(map_into_keys_values)]
use itertools::Itertools;

fn main() {
    let (p1, p2) = include_str!("../input")
        .split("\n\n")
        .fold((0, 0), |acc, g| {
            let mut counts = g.chars().counts();
            let people = 1 + counts.remove(&'\n').unwrap_or_default();
            let p1 = counts.len();
            let p2 = counts.into_values().filter(|&n| n == people).count();
            (acc.0 + p1, acc.1 + p2)
        });

    println!("p1= {}", p1);
    println!("p2= {}", p2);
}
