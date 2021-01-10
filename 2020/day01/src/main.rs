use itertools::Itertools;

pub fn main() {
    let input: Vec<_> = include_str!("../input")
        .lines()
        .flat_map(|i| i.parse::<usize>())
        .collect();

    println!(
        "p1= {}",
        input
            .iter()
            .tuple_combinations()
            .find(|&(a, b)| a + b == 2020)
            .map(|(a, b)| a * b)
            .unwrap()
    );

    println!(
        "p2= {}",
        input
            .iter()
            .tuple_combinations()
            .find(|&(a, b, c)| a + b + c == 2020)
            .map(|(a, b, c)| a * b * c)
            .unwrap()
    );
}
