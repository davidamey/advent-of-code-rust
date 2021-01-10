use regex::Regex;

pub fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();

    let pwds: Vec<_> = include_str!("../input")
        .lines()
        .map(|l| {
            let cs = re.captures(l).unwrap();
            (
                cs[1].parse::<usize>().unwrap(),
                cs[2].parse::<usize>().unwrap(),
                cs[3].to_string(),
                cs[4].to_string(),
            )
        })
        .collect();

    println!(
        "p1= {}",
        pwds.iter()
            .filter(|(i, j, c, w)| {
                let len = w.matches(c).count();
                len >= *i && len <= *j
            })
            .count()
    );

    println!(
        "p2= {}",
        pwds.iter()
            .filter(|(i, j, c, w)| {
                let b = c.as_bytes()[0];
                let bs = w.as_bytes();
                (bs[*i - 1] == b || bs[*j - 1] == b) && bs[*i - 1] != bs[*j - 1]
            })
            .count()
    );
}
