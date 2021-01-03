use crate::input;
use std::collections::HashMap;

pub fn day1() {
    let contents = input::load_day_file("day1.txt");
    let mut map = HashMap::new();
    let mut first = true;
    for (i, c) in contents.trim().chars().enumerate() {
        *map.entry(c).or_insert(0) += 1;

        if map.get(&'(').unwrap_or(&0) - map.get(&')').unwrap_or(&0) == -1 && first {
            println!("First time in basement: {} ", i + 1);
            first = false;
        }
    }
    println!(
        "Floor 🏠: {} ",
        map.get(&'(').unwrap() - map.get(&')').unwrap()
    );
}
