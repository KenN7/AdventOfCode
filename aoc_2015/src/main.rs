mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
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
        // Day::new("Day 14", day14::day14),
        // Day::new("Day 15", day15::day15),
        // Day::new("Day 16", day16::day16),
        // Day::new("Day 17", day17::day17),
        // Day::new("Day 18", day18::day18),
        // Day::new("Day 19", day19::day19),
        // Day::new("Day 20", day20::day20),
        // Day::new("Day 21", day21::day21),
        // Day::new("Day 22", day22::day22),
        // Day::new("Day 23", day23::day23),
        // Day::new("Day 24", day24::day24),
        Day::new("Day 25", day25::day25),
    ];
    for day in days.iter() {
        day.run();
    }
}
