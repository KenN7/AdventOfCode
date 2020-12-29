// use itertools::Itertools;
// use regex::Regex;
use std::collections::BTreeMap;
use std::env;
use std::fs;

fn puzzle1(data: &[&str], stop: usize) {
    let mut nums: BTreeMap<i64, (i64, i64)> = data
        .iter()
        .take(data.len() - 1)
        .enumerate()
        .map(|(i, x)| (x.parse::<i64>().unwrap(), (i as i64 + 1, 0)))
        .collect();
    let mut last_num = data.last().unwrap().parse::<i64>().unwrap();
    println!("{:?}", nums);
    for turn in nums.len() + 2..stop + 1 {
        if nums.contains_key(&last_num) {
            nums.insert(last_num, (turn as i64 - 1, nums.get(&last_num).unwrap().0));
            last_num = nums.get(&last_num).unwrap().0 - nums.get(&last_num).unwrap().1;
        } else {
            nums.insert(last_num, (turn as i64 - 1, nums.get(&0).unwrap().0));
            last_num = 0;
        }
        // println!("turn: {:?} spoken: {:?}", turn, last_num);
    }
    println!("last: {:?}", last_num);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let stop: usize = args[2].parse().unwrap();
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = content.trim().split(',').collect();

    puzzle1(&data, stop);
    // println!("{:?}", data);
    // puzzle2(&data);
}
