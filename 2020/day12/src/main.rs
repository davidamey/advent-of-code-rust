fn main() {
    let instructions: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| (l.chars().nth(0).unwrap(), l[1..].parse::<isize>().unwrap()))
        .collect();

    println!(
        "p1= {}",
        manhattan(
            instructions
                .iter()
                .fold(((0, 0), (1, 0)), |(ship, dir), (i, amt)| match i {
                    'N' => ((ship.0, ship.1 - amt), dir),
                    'S' => ((ship.0, ship.1 + amt), dir),
                    'E' => ((ship.0 + amt, ship.1), dir),
                    'W' => ((ship.0 - amt, ship.1), dir),
                    'L' => (ship, (0..*amt).step_by(90).fold(dir, |d, _| (d.1, -d.0))),
                    'R' => (ship, (0..*amt).step_by(90).fold(dir, |d, _| (-d.1, d.0))),
                    'F' => ((ship.0 + dir.0 * amt, ship.1 + dir.1 * amt), dir),
                    _ => panic!("not a valid dir"),
                })
                .0
        )
    );

    println!(
        "p2= {}",
        manhattan(
            instructions
                .iter()
                .fold(((0, 0), (10, -1)), |(ship, wp), (i, amt)| match i {
                    'N' => (ship, (wp.0, wp.1 - amt)),
                    'S' => (ship, (wp.0, wp.1 + amt)),
                    'E' => (ship, (wp.0 + amt, wp.1)),
                    'W' => (ship, (wp.0 - amt, wp.1)),
                    'L' => (ship, (0..*amt).step_by(90).fold(wp, |d, _| (d.1, -d.0))),
                    'R' => (ship, (0..*amt).step_by(90).fold(wp, |d, _| (-d.1, d.0))),
                    'F' => ((ship.0 + wp.0 * amt, ship.1 + wp.1 * amt), wp),
                    _ => panic!("not a valid dir"),
                })
                .0
        )
    );
}

fn manhattan(v: (isize, isize)) -> isize {
    v.0.abs() + v.1.abs()
}
