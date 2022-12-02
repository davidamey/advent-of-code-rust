mod year2019;
mod year2020;
mod year2021;
mod year2022;

pub fn exec(year: i32, day: i32) {
    match year {
        2019 => year2019::exec_day(day),
        2020 => year2020::exec_day(day),
        2021 => year2021::exec_day(day),
        2022 => year2022::exec_day(day),
        _ => println!("Year {} doesn't have solutions yet :(", year),
    }
}
