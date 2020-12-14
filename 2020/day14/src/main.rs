use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

fn puzzle1(data: &[&str]) {
    let re = Regex::new(r"\[(\d+)\] = (\d+)").unwrap();
    let mut masks = (0, 0);
    let mut memory = HashMap::new();
    for line in data.iter() {
        if let Some(mask) = line.strip_prefix("mask = ") {
            masks = (
                u64::from_str_radix(&mask.replace('X', "0"), 2).unwrap(),
                u64::from_str_radix(&mask.replace('X', "1"), 2).unwrap(),
            );
        } else if let Some(mem) = line.strip_prefix("mem") {
            let caps = re.captures(mem).unwrap();
            let addr: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let num: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
            memory.insert(addr, (num & masks.1) | masks.0);
        }
    }
    // println!("{:?}", memory);
    println!("Puzzle 1: {}", memory.values().sum::<u64>());
}

fn puzzle2(data: &[&str]) {
    let re = Regex::new(r"\[(\d+)\] = (\d+)").unwrap();
    let mut mask = "";
    let mut memory = HashMap::new();
    for line in data.iter() {
        if let Some(res_mask) = line.strip_prefix("mask = ") {
            mask = res_mask;
        } else if let Some(mem) = line.strip_prefix("mem") {
            let caps = re.captures(mem).unwrap();
            let (addr, num): (u64, u64) = (
                caps.get(1).unwrap().as_str().parse().unwrap(),
                caps.get(2).unwrap().as_str().parse().unwrap(),
            );
            let x_ind: Vec<usize> = mask
                .match_indices('X')
                .map(|x| mask.len() - 1 - x.0)
                .collect();
            let m = u64::from_str_radix(&mask.replace('X', "1"), 2).unwrap();
            for res in gen_comb(addr | m, x_ind).iter() {
                memory.insert(res.to_owned(), num);
            }
        }
    }
    // println!("{:?}", memory);
    println!("puzzle 2: {}", memory.values().sum::<u64>());
}

fn gen_comb(mut n_int: u64, x_ind: Vec<usize>) -> Vec<u64> {
    let mut n_list = vec![n_int];
    n_int -= x_ind.iter().map(|x| 2u64.pow(*x as u32)).sum::<u64>();
    n_list.push(n_int);
    // println!("mask_int: {:?}", n_int);
    // println!("x_ind: {:?}", x_ind);
    for i in 1..x_ind.len() {
        n_list.extend(
            x_ind
                .iter()
                .map(|x| 2u64.pow(*x as u32))
                .combinations(i)
                .map(|c| c.iter().sum::<u64>() + n_int),
        )
    }
    // println!("list of addr: {:?}", n_list);
    n_list
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = content.trim().split('\n').collect();

    puzzle1(&data);
    puzzle2(&data);
}
