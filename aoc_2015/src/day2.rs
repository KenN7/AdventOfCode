use itertools::Itertools;
use std::env;
use std::fs;

pub fn day2() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<Vec<i32>> = contents
        .lines()
        .map(|x| x.split('x').map(|x| x.parse().unwrap()).sorted().collect())
        .collect();
    // Vec l x w x h, sorted
    // 2*l*w + 2*w*h + 2*h*l + smallest area

    let total_ft: Vec<i32> = data
        .iter()
        .map(|x| 2 * x[0] * x[1] + 2 * x[1] * x[2] + 2 * x[2] * x[0] + x[0] * x[1])
        .collect();
    println!(
        "Total wrapping paper size needed 🎁: {} ft",
        total_ft.iter().sum::<i32>()
    );

    let total_rb: Vec<i32> = data
        .iter()
        .map(|x| x[0] * 2 + x[1] * 2 + x[0] * x[1] * x[2])
        .collect();
    println!(
        "Total ribbon needed 🎀: {} ft",
        total_rb.iter().sum::<i32>()
    );
}
