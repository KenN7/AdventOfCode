use regex::Regex;
use std::env;
use std::fs;

fn puzzle1(contents: &String) {
    let re = Regex::new(r"(\d*)-(\d*) (\w): (\w*)").unwrap();
    let mut valid = 0;
    for line in contents.lines() {
        for cap in re.captures_iter(line) {
            let count = count_let(&cap[4], &cap[3]);
            if count >= (&cap[1]).parse().unwrap() && count <= (&cap[2]).parse().unwrap() {
                valid += 1;
            }
        }
    }
    println!("Valid: {}", valid);
}

fn puzzle2(contents: &String) {
    let re = Regex::new(r"(\d*)-(\d*) (\w): (\w*)").unwrap();
    let mut valid = 0;
    for line in contents.lines() {
        for cap in re.captures_iter(line) {
            valid += count_let2(
                &cap[4],
                (&cap[3]).chars().nth(0).unwrap(),
                (&cap[1]).parse().unwrap(),
                (&cap[2]).parse().unwrap(),
            );
        }
    }
    println!("Valid: {}", valid);
}

fn count_let(line: &str, i: &str) -> i32 {
    let mut count = 0;
    for item in line.chars() {
        if item == i.chars().nth(0).unwrap() {
            count = count + 1;
        }
    }
    count
}

fn count_let2(line: &str, letter: char, i1: usize, i2: usize) -> i32 {
    // for &item in bytes.iter() {
    if (line.chars().nth(i1 - 1).unwrap() == letter)
        ^ (line.chars().nth(i2 - 1).unwrap() == letter)
    {
        return 1;
    }
    return 0;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    puzzle1(&contents);
    puzzle2(&contents);
}
