use crate::input;
use itertools::Itertools;
use onig::Regex;
use std::collections::{HashMap, HashSet};

fn happiness(table: &Vec<&&str>, map: &HashMap<(&str, &str), i32>) -> input::Result<i32> {
    let mut happiness = 0;
    let len = table.len();
    for i in 0..len {
        happiness += map
            .get(&(table[i], table[(i + 1).rem_euclid(len)]))
            .ok_or("Entry not found in map")?;
        happiness += map
            .get(&(
                table[i],
                table[(i as isize - 1).rem_euclid(len as isize) as usize],
            ))
            .ok_or("Entry not found in map")?;
    }
    Ok(happiness)
}

pub fn day13() -> input::Result<()> {
    let contents = input::load_day_file("day13.txt");
    let re =
        Regex::new(r"(\w+) would (lose|gain) (\d+) happiness units by sitting next to (\w+).")?;

    let mut map = HashMap::new();
    let mut guests = HashSet::new();
    for line in contents.lines() {
        let cap = re.captures(line).ok_or("Regex not matching")?;
        let mut sign = 1;
        if cap.at(2).unwrap() == "lose" {
            sign = -1;
        }
        map.insert(
            (cap.at(1).unwrap(), cap.at(4).unwrap()),
            cap.at(3).unwrap().parse::<i32>().unwrap() * sign,
        );
        guests.insert(cap.at(1).unwrap());
    }

    let happy = guests
        .iter()
        .permutations(guests.len())
        .map(|p| happiness(&p, &map).unwrap())
        .collect_vec();

    println!(
        "Max 🤗 Happiness of table 🍽️:: {}",
        happy.iter().max().unwrap()
    );

    for g in guests.iter() {
        map.insert(("A", g), 0);
        map.insert((g, "A"), 0);
    }
    guests.insert("A");
    let happy = guests
        .iter()
        .permutations(guests.len())
        .map(|p| happiness(&p, &map).unwrap())
        .collect_vec();

    println!(
        "Max 🤗 Happiness of table 🍽️  (with you included): {}",
        happy.iter().max().unwrap()
    );

    Ok(())
}
