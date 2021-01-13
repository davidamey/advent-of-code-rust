use std::collections::HashMap;

fn main() {
    let mut adaptors: Vec<usize> = include_str!("../input")
        .lines()
        .flat_map(|l| l.parse())
        .collect();
    adaptors.sort_unstable();

    let (d1, d3) = adaptors.windows(2).fold(
        match adaptors[0] {
            1 => (1, 1),
            3 => (0, 2),
            _ => (0, 1),
        },
        |acc, w| match w[1] - w[0] {
            1 => (acc.0 + 1, acc.1),
            3 => (acc.0, acc.1 + 1),
            _ => unreachable!(),
        },
    );
    println!("p1= {}", d1 * d3);

    let mut cache: HashMap<usize, usize> = HashMap::new();
    cache.insert(*adaptors.last().unwrap(), 1);
    println!("p2= {}", part2(0, adaptors.as_slice(), &mut cache));
}

fn part2(joltage: usize, pool: &[usize], cache: &mut HashMap<usize, usize>) -> usize {
    if let Some(&v) = cache.get(&joltage) {
        return v;
    }

    let score = pool
        .iter()
        .take(3)
        .enumerate()
        .filter(|&(_, j)| {
            let d = j - joltage;
            d >= 1 && d <= 3
        })
        .fold(0, |acc, (i, &j)| acc + part2(j, &pool[i + 1..], cache));

    cache.insert(joltage, score);
    score
}
