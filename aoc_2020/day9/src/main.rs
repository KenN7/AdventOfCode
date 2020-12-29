// use regex::Regex;
use itertools::Itertools;
use std::env;
use std::fs;

fn puzzle(data: &[usize], len: usize) {
    let iter = data.iter().skip(len);
    let mut inv = 0;
    for (i, nb) in iter.enumerate() {
        let perm = data[i..i + len].iter().combinations(2);

        let valid = perm.map(|x| x[0]+x[1] == *nb).any(|x| x);
        if !valid {
            println!("Not valid: {}", nb);
            inv = *nb;
            break;
        }
    }
    'out: for i in 2..data.len() {
        for chunk in data.windows(i) {
            if chunk.iter().sum::<usize>() == inv {
                println!(
                    "min: {}, max: {}, sum: {}",
                    chunk.iter().min().unwrap(),
                    chunk.iter().max().unwrap(),
                    chunk.iter().min().unwrap() + chunk.iter().max().unwrap()
                );
                break 'out;
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let data: Vec<usize> = content
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    puzzle(&data, 25);
}
