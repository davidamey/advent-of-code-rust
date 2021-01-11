use regex::Regex;

pub fn main() {
    let (p1, p2) = include_str!("../input")
        .split("\n\n")
        .fold((0, 0), |acc, p| {
            let (v1, v2) = validate_pass(p);
            (
                acc.0 + if v1 { 1 } else { 0 },
                acc.1 + if v2 { 1 } else { 0 },
            )
        });

    println!("p1= {}", p1);
    println!("p2= {}", p2);
}

fn validate_pass(pass: &str) -> (bool, bool) {
    let re_fields = Regex::new(r"(byr|iyr|eyr|hgt|hcl|ecl|pid):(\S*)").unwrap();
    let re_hex_colour = Regex::new(r"#[a-z0-9]{6}").unwrap();
    let re_eye_colour = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();

    let num_fields = re_fields.find_iter(pass).count();
    if num_fields != 7 {
        return (false, false);
    }

    for cs in re_fields.captures_iter(pass) {
        let (k, v) = (cs.get(1).unwrap().as_str(), cs.get(2).unwrap().as_str());
        let valid = match k {
            "byr" => is_number_between(v, 1920, 2002),
            "iyr" => is_number_between(v, 2010, 2020),
            "eyr" => is_number_between(v, 2020, 2030),
            "hgt" => {
                if &v[v.len() - 2..] == "cm" {
                    is_number_between(&v[0..v.len() - 2], 150, 193)
                } else {
                    is_number_between(&v[0..v.len() - 2], 59, 76)
                }
            }
            "hcl" => re_hex_colour.is_match(v),
            "ecl" => re_eye_colour.is_match(v),
            "pid" => v.len() == 9 && is_number_between(v, 0, 0),
            _ => panic!("unknown field"),
        };

        if !valid {
            return (true, false);
        }
    }

    (true, true)
}

fn is_number_between(num: &str, min: i32, max: i32) -> bool {
    let n: i32 = match num.parse() {
        Ok(x) => x,
        Err(_) => return false,
    };

    if min == 0 && max == 0 {
        return true;
    }

    n >= min && n <= max
}
