mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

mod input;

use input::Day;

fn main() {
    let days = vec![
        // Day::new("Day 1", day1::day1),
        // Day::new("Day 2", day2::day2),
        // Day::new("Day 3", day3::day3),
        // Day::new("Day 4", day4::day4),
        // Day::new("Day 5", day5::day5),
        // Day::new("Day 6", day6::day6),
        // Day::new("Day 7", day7::day7),
        // Day::new("Day 8", day8::day8),
        // Day::new("Day 9", day9::day9),
        // Day::new("Day 10", day10::day10),
        // Day::new("Day 11", day11::day11),
        // Day::new("Day 12", day12::day12),
        // Day::new("Day 13", day13::day13),
        Day::new("Day 14", day14::day14),
    ];
    for day in days.iter() {
        day.run();
    }
}
