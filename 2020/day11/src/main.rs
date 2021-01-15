type Point = (usize, usize);

fn main() {
    // let grid = include_bytes!("../example")
    let grid = include_bytes!("../input")
        .split(|&b| b == b'\n')
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();

    let (width, height) = (grid[0].len() as isize, grid.len() as isize);

    let seats: Vec<Point> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &b)| b == b'L')
                .map(move |(x, _)| (x, y))
        })
        .collect();

    // === part1 ===
    #[rustfmt::skip]
    let seats_with_neighbours: Vec<(Point, Vec<Point>)> =
        seats.iter().map(|&p| (p,
            (0..9)
                .filter(|d| d != &4)
                .map(|d| ((p.0 as isize) + d % 3 - 1, (p.1 as isize) + d / 3 - 1))
                .filter(|&(x, y)| x >= 0 && y >= 0 && x < width && y < height)
                .map(|(x, y)| (x as usize, y as usize))
                .collect::<Vec<_>>()
        )).collect();

    println!(
        "p1= {}",
        evolve(&grid, seats_with_neighbours, |s, occupied| {
            if s == b'L' && occupied == 0 {
                b'#'
            } else if s == b'#' && occupied >= 4 {
                b'L'
            } else {
                s
            }
        })
    );

    // === part2 ===
    #[rustfmt::skip]
    let seats_with_neighbours: Vec<(Point, Vec<Point>)> =
        seats.iter().map(|&p| (p,
            (0..9)
                .filter(|d| d != &4)
                .map(|d| (d % 3 - 1, d / 3 - 1))
                .filter_map(|(dx, dy)| (1..)
                    .map(move |i| ((p.0 as isize) + dx * i, (p.1 as isize) + dy * i))
                    .take_while(|&(x, y)| x >= 0 && y >= 0 && x < width && y < height)
                    .map(|(x, y)| (x as usize, y as usize))
                    .filter(|(x, y)| grid[*y][*x] != b'.')
                    .next()
                )
                .collect::<Vec<_>>()
        )).collect();

    println!(
        "p2= {}",
        evolve(&grid, seats_with_neighbours, |s, occupied| {
            if s == b'L' && occupied == 0 {
                b'#'
            } else if s == b'#' && occupied >= 5 {
                b'L'
            } else {
                s
            }
        })
    );
}

fn evolve<F>(
    start: &Vec<Vec<u8>>,
    seats_with_neighbours: Vec<(Point, Vec<Point>)>,
    evolve_fn: F,
) -> usize
where
    F: Fn(u8, usize) -> u8,
{
    let mut cur = start.clone();
    let mut nxt = start.clone();

    while {
        for ((x, y), neighbours) in &seats_with_neighbours {
            let occupied = neighbours
                .iter()
                .filter(|&(x, y)| cur[*y][*x] == b'#')
                .count();

            nxt[*y][*x] = evolve_fn(cur[*y][*x], occupied);
        }

        std::mem::swap(&mut cur, &mut nxt);
        nxt != cur
    } {}

    cur.iter()
        .fold(0, |acc, r| acc + r.iter().filter(|s| s == &&b'#').count())
}
