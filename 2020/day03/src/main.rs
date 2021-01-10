pub fn main() {
    let slope: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| l.as_bytes())
        .collect();

    let p1 = traverse(&slope, 3, 1);
    let p2 = p1
        * traverse(&slope, 1, 1)
        * traverse(&slope, 5, 1)
        * traverse(&slope, 7, 1)
        * traverse(&slope, 1, 2);

    println!("p1= {}", p1);
    println!("p2= {}", p2);
}

fn traverse(slope: &Vec<&[u8]>, dx: usize, dy: usize) -> usize {
    let mut trees = 0;
    let mut x = 0;
    let mut y = 0;

    while y < slope.len() {
        if slope[y][x] == b'#' {
            trees += 1;
        }
        x = (x + dx) % slope[0].len();
        y += dy
    }

    trees
}
