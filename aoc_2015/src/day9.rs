use crate::input;
use ndarray::Array2;
use onig::Regex;
use std::collections::HashMap;

pub fn day9() -> input::Result<()> {
    let contents = input::load_day_file("day9.txt");

    let re_v = Regex::new(r"(\w*) to (\w*) = (\d+)")?;

    let mut names = HashMap::new();
    let mut indexes = Vec::new();

    let size = ((1f64 + (1f64 + 8f64 * contents.lines().count() as f64).sqrt()) / 2f64) as usize;
    let mut graph: Array2<u32> = Array2::zeros((size, size));

    let mut i = 0;
    for line in contents.lines() {
        let cap = re_v.captures(line).ok_or("Regex didnt match")?;
        let n1 = *names
            .entry(cap.at(1).ok_or("No group 1")?)
            .or_insert_with(|| {
                let old = i;
                indexes.push(cap.at(1).unwrap());
                i += 1;
                old
            });
        let n2 = *names
            .entry(cap.at(2).ok_or("No group 2")?)
            .or_insert_with(|| {
                let old = i;
                indexes.push(cap.at(2).unwrap());
                i += 1;
                old
            });
        let dist = cap.at(3).ok_or("No group 3")?.parse::<u32>()?;
        graph[[n1, n2]] = dist;
        graph[[n2, n1]] = dist;
    }

    // println!("{:?}", graph);
    // println!("{:?}", names);
    // println!("{:?}", indexes);

    let mut queue: Vec<_> = indexes.iter().map(|&x| (vec![x], 0)).collect();
    let mut paths = Vec::new();
    while !queue.is_empty() {
        // next = queue.remove(0);
        let next = queue.pop().unwrap();
        queue.extend(names.iter().filter(|x| !next.0.contains(x.0)).map(|x| {
            let mut new_list = next.0.clone();
            new_list.push(x.0);
            (
                new_list,
                next.1
                    + graph[[
                        *names.get(next.0.last().unwrap()).unwrap(),
                        *names.get(x.0).unwrap(),
                    ]],
            )
        }));
        if next.0.len() == size {
            paths.push(next);
        }
    }

    let min = paths.iter().min_by_key(|x| x.1).unwrap();
    let max = paths.iter().max_by_key(|x| x.1).unwrap();
    println!(
        "🚅 Shortest path is{}. \nIt is of length: {}",
        min.0.iter().fold(String::new(), |a, &v| a + " -> " + v),
        min.1
    );
    println!(
        "🚂 Longest path is{}. \nIt is of length: {}",
        max.0.iter().fold(String::new(), |a, &v| a + " -> " + v),
        max.1
    );

    Ok(())
}
