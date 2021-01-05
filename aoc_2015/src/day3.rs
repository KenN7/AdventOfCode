use crate::input;
use std::collections::HashMap;

pub fn day3() -> input::Result<()> {
    let contents = input::load_day_file("day3.txt");
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut pos = (0, 0);

    for c in contents.chars() {
        match c {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => (),
        };
        *map.entry(pos).or_insert(0) += 1;
    }
    println!("Santa 🎅 delivered {} 🏠 houses.", map.iter().count());

    let mut pos1 = (0, 0);
    let mut pos2 = (0, 0);
    let mut pos;
    map.clear();
    for (i, c) in contents.chars().enumerate() {
        if i % 2 == 0 {
            pos = &mut pos1;
        } else {
            pos = &mut pos2;
        }
        match c {
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 += 1,
            'v' => pos.1 -= 1,
            _ => (),
        };
        *map.entry(pos1).or_insert(0) += 1;
        *map.entry(pos2).or_insert(0) += 1;
    }
    println!(
        "Santa 🎅 and Robo-Santa 🤖 delivered {} 🏠 houses.",
        map.iter().count()
    );
    Ok(())
}
