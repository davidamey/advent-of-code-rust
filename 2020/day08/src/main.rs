use std::collections::HashSet;

fn main() {
    let instructions = include_str!("../input")
        .lines()
        .map(|l| (&l[0..3], l[4..].parse::<isize>().unwrap()))
        .collect::<Vec<_>>();

    let (p1, _) = run(&instructions, instructions.len() + 2);
    println!("p1= {}", p1);

    let mut p2 = 0;
    for (i, &(op, _)) in instructions.iter().enumerate() {
        if op == "acc" {
            continue;
        }

        let (r, term) = run(&instructions, i);
        if term {
            p2 = r;
            break;
        }
    }
    println!("p2= {}", p2);
}

fn run(instructions: &Vec<(&str, isize)>, switch_instruct: usize) -> (isize, bool) {
    let mut acc = 0;
    let mut i = 0;
    let mut seen = HashSet::new();

    while !seen.contains(&i) && i < instructions.len() {
        seen.insert(i);
        let (op, arg) = instructions[i];
        match op {
            "nop" => {
                if i == switch_instruct {
                    i = add(i, arg);
                } else {
                    i += 1
                }
            }
            "acc" => {
                acc += arg;
                i += 1;
            }
            "jmp" => {
                if i == switch_instruct {
                    i += 1;
                } else {
                    i = add(i, arg);
                }
            }
            _ => panic!("unknown op"),
        }
    }

    (acc, i >= instructions.len())
}

#[inline(always)]
fn add(a: usize, b: isize) -> usize {
    ((a as isize) + b) as usize
}
