use itertools::Itertools;

const PREAMBLE: usize = 25;

fn main() {
    let ints: Vec<usize> = include_str!("../input")
        .lines()
        .flat_map(|l| l.parse())
        .collect();

    let &p1 = ints
        .iter()
        .enumerate()
        .skip(PREAMBLE)
        .find(|&(i, &x)| {
            !ints[i - PREAMBLE..i]
                .iter()
                .tuple_combinations()
                .any(|(a, b)| a + b == x)
        })
        .unwrap()
        .1;

    let (mut i, mut j, mut sum) = (0, 1, ints[0] + ints[1]);
    while sum != p1 {
        while sum < p1 {
            j += 1;
            sum += ints[j];
        }
        while sum > p1 {
            sum -= ints[i];
            i += 1;
        }
    }
    let (min, max) = ints[i..=j].iter().minmax().into_option().unwrap();

    println!("p1= {}", p1);
    println!("p2= {}", min + max);
}
