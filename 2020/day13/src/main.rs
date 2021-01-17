fn main() {
    let (earliest, notes) = {
        let lines: Vec<_> = include_str!("../input").lines().collect();
        (lines[0].parse::<i64>().unwrap(), lines[1])
    };

    let buses: Vec<_> = notes
        .split(",")
        .enumerate()
        .filter(|(_, n)| n != &"x")
        .map(|(i, n)| (i as i64, n.parse::<i64>().unwrap()))
        .collect();

    println!(
        "p1= {}",
        buses
            .iter()
            .map(|(_, b)| {
                let t = b * ((earliest + b - 1) / b);
                (b, t, b * (t - earliest))
            })
            .min_by(|x, y| x.1.cmp(&y.1))
            .unwrap()
            .2
    );

    let product: i64 = buses.iter().map(|(_, b)| b).product();
    println!(
        "p2= {}",
        buses
            .iter()
            .map(|(i, b)| { (b, (-i).rem_euclid(*b)) })
            .fold(0, |sum, (m, r)| {
                let p = product / m;
                sum + (r * p * mod_inv(p, *m))
            })
            % product
    );
}

fn mod_inv(a: i64, modulus: i64) -> i64 {
    let mut mn = (modulus, a);
    let mut xy = (0, 1);

    while mn.1 != 0 {
        xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
        mn = (mn.1, mn.0 % mn.1);
    }

    while xy.0 < 0 {
        xy.0 += modulus;
    }
    xy.0
}
