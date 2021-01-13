use crate::input;
use onig::Regex;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

fn score(ingrs: &Vec<u32>, map: &HashMap<String, Vec<i32>>) -> input::Result<(u32, i32)> {
    let mut scores = vec![0; map.values().next().ok_or("Empty map")?.len()];
    for (j, ingr) in map.iter().enumerate() {
        for (i, v) in ingr.1.iter().enumerate() {
            scores[i] += *v * i32::try_from(ingrs[j])?;
        }
    }
    if scores.iter().rev().skip(1).any(|&x| x < 0) {
        return Ok((0, 0));
    }
    Ok((
        scores.iter().rev().skip(1).product::<i32>().try_into()?,
        *scores.last().ok_or("Empty scores vector")?,
    ))
}

pub fn day15() -> input::Result<()> {
    let contents = input::load_day_file("day15.txt");

    let re = Regex::new(r"(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();

    let mut map = HashMap::new();
    for line in contents.lines() {
        let cap = re.captures(line).unwrap();
        let ingr = cap.iter().collect::<Option<Vec<_>>>().unwrap();
        map.insert(
            ingr.get(1).unwrap().to_string(),
            ingr.iter()
                .skip(2)
                .map(|x| x.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    println!("{:?}", map);

    fn recur(used: Vec<u32>, id: usize, map: &HashMap<String, Vec<i32>>) -> Vec<Vec<u32>> {
        if used.iter().sum::<u32>() == 100 {
            return vec![used];
        }
        let mut new_used = Vec::new();
        for (i, _ingr) in map.keys().enumerate().skip(id) {
            let mut used2 = used.clone();
            used2[i] += 1;
            new_used.extend(recur(used2, i, &map));
        }
        new_used
    }

    let used = recur(vec![0; map.keys().len()], 0, &map);
    let scores = used
        .iter()
        .map(|x| score(x, &map))
        .collect::<Result<Vec<_>, _>>()?;

    println!(
        "Product 🍪 of used ingredients: {:?}",
        scores.iter().max().unwrap().0
    );

    println!(
        "Product for 🍪 cookies of 500kcal: {:?}",
        scores.iter().filter(|x| x.1 == 500).max().unwrap().0
    );

    Ok(())
}
