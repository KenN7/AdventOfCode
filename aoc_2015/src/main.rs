mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

mod input;

use input::Day;

fn main() {
    let days = vec![
        Day::new("Day 1", day1::day1),
        Day::new("Day 2", day2::day2),
        Day::new("Day 3", day3::day3),
        Day::new("Day 4", day4::day4),
        Day::new("Day 5", day5::day5),
        Day::new("Day 6", day6::day6),
    ];
    for day in days.iter() {
        day.run();
    }
}
