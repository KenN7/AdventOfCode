use crate::input;
use onig::Regex;
use std::collections::HashMap;

pub fn day14() -> input::Result<()> {
    let contents = input::load_day_file("day14.txt");
    let re = Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    const TIME: i32 = 2503;

    let mut deers_map = HashMap::new();
    let mut deers = Vec::new();
    for deer in contents.lines() {
        let cap = re.captures(deer).unwrap();
        let break_time = cap.at(4).unwrap().parse::<i32>().unwrap();
        let speed = cap.at(2).unwrap().parse::<i32>().unwrap();
        let speed_time = cap.at(3).unwrap().parse::<i32>().unwrap();
        let dist = TIME.div_euclid(break_time + speed_time) * speed * speed_time
            + speed * std::cmp::min(speed_time, TIME.rem_euclid(break_time + speed_time));
        deers.push(dist);
        deers_map.insert(cap.at(1).unwrap(), (speed, speed_time, break_time));
    }

    println!(
        "The best 🦌 deer has 🧭 gone {} km !",
        deers.iter().max().unwrap()
    );

    // println!("{:?}", deers_map);
    let mut deers_scores = HashMap::new();
    let mut deers_dist = HashMap::new();

    for t in 0..TIME {
        for deer in deers_map.keys() {
            let (s_t, b_t) = (
                deers_map.get(deer).unwrap().1,
                deers_map.get(deer).unwrap().2,
            );
            if t.rem_euclid(s_t + b_t) < s_t {
                *deers_dist.entry(deer).or_insert(0) += deers_map.get(deer).unwrap().0;
            }
        }

        let best_dist = deers_dist.iter().max_by_key(|x| x.1).unwrap().1.clone();
        deers_dist
            .iter()
            .filter(|x| *x.1 == best_dist)
            .map(|x| x.0)
            .for_each(|x| *deers_scores.entry(*x).or_insert(0) += 1);
    }

    // println!("{:?}", deers_scores);
    println!(
        "The best 🦌 deer has {} points !",
        deers_scores.values().max().unwrap()
    );

    Ok(())
}
