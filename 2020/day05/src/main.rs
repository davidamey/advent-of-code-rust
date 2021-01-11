fn main() {
    let mut seats = include_str!("../input")
        .lines()
        .map(|l| {
            let (row, col) = l.split_at(7);
            8 * decode(row) + decode(col)
        })
        .collect::<Vec<_>>();

    seats.sort_unstable();

    println!("p1= {}", seats.iter().last().unwrap());
    println!(
        "p2= {}",
        seats.windows(2).find(|w| w[0] == w[1] - 2).unwrap()[0] + 1
    );
}

fn decode(code: &str) -> usize {
    code.as_bytes().iter().fold(0, |acc, b| match b {
        b'F' | b'L' => acc << 1,
        b'B' | b'R' => (acc << 1) + 1,
        _ => panic!("unexpected char {}", b),
    })
}
