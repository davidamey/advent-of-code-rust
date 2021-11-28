use crate::day::Day;

const YEAR: i32 = 2019;

mod day01;
// mod day02;
// mod day03;
// mod day04;
// mod day05;
// mod day06;
// mod day07;
// mod day08;
// mod day09;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

pub fn exec_day(d: i32) {
    match d {
        1 => day01::AoC {}.solve(YEAR, d),
        //  2 => day02::AoC {}.solve(YEAR, d),
        //  3 => day03::AoC {}.solve(YEAR, d),
        //  4 => day04::AoC {}.solve(YEAR, d),
        //  5 => day05::AoC {}.solve(YEAR, d),
        //  6 => day06::AoC {}.solve(YEAR, d),
        //  7 => day07::AoC {}.solve(YEAR, d),
        //  8 => day08::AoC {}.solve(YEAR, d),
        //  9 => day09::AoC {}.solve(YEAR, d),
        // 10 => day10::AoC {}.solve(YEAR, d),
        // 11 => day11::AoC {}.solve(YEAR, d),
        // 12 => day12::AoC {}.solve(YEAR, d),
        // 13 => day13::AoC {}.solve(YEAR, d),
        // 14 => day14::AoC {}.solve(YEAR, d),
        // 15 => day15::AoC {}.solve(YEAR, d),
        // 16 => day16::AoC {}.solve(YEAR, d),
        // 17 => day17::AoC {}.solve(YEAR, d),
        // 18 => day18::AoC {}.solve(YEAR, d),
        // 19 => day19::AoC {}.solve(YEAR, d),
        // 20 => day20::AoC {}.solve(YEAR, d),
        // 21 => day21::AoC {}.solve(YEAR, d),
        // 22 => day22::AoC {}.solve(YEAR, d),
        // 23 => day22::AoC {}.solve(YEAR, d),
        // 24 => day22::AoC {}.solve(YEAR, d),
        // 25 => day22::AoC {}.solve(YEAR, d),
        d => println!("{} day {} hasn't been solved yet :(", YEAR, d),
    }
}
