fn main() {
    let instructions: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| l.split_at(1))
        .map(|(d, v)| (d.chars().nth(0).unwrap(), v.parse::<isize>().unwrap()))
        .collect();

    let mut s1 = (0, 0);
    let mut s2 = (0, 0);
    let mut dir = (1, 0);
    let mut wp = (10, -1);

    for (i, amt) in &instructions {
        match i {
            'N' => {
                s1.1 -= amt;
                wp.1 -= amt;
            }
            'S' => {
                s1.1 += amt;
                wp.1 += amt;
            }
            'E' => {
                s1.0 += amt;
                wp.0 += amt;
            }
            'W' => {
                s1.0 -= amt;
                wp.0 -= amt;
            }
            'L' => {
                for _ in (0..*amt).step_by(90) {
                    dir = (dir.1, -dir.0);
                    wp = (wp.1, -wp.0);
                }
            }
            'R' => {
                for _ in (0..*amt).step_by(90) {
                    dir = (-dir.1, dir.0);
                    wp = (-wp.1, wp.0);
                }
            }
            'F' => {
                s1.0 += dir.0 * amt;
                s1.1 += dir.1 * amt;
                s2.0 += wp.0 * amt;
                s2.1 += wp.1 * amt;
            }
            _ => unreachable!("not a valid dir"),
        }
    }

    println!("p1= {}", manhattan(s1));
    println!("p2= {}", manhattan(s2));
}

fn manhattan(v: (isize, isize)) -> isize {
    v.0.abs() + v.1.abs()
}
