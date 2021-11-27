mod year2019;
mod year2020;

pub fn exec(year: i32, day: i32) {
    match year {
        2019 => year2019::exec_day(day),
        2020 => year2020::exec_day(day),
        _ => println!("Year {} doesn't have solutions yet :(", year),
    }
}
