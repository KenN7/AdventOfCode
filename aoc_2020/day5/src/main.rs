use std::env;
use std::fs;

fn puzzle1(passes: &Vec<&str>) -> Vec<u32> {
    let mut seats_id: Vec<u32> = Vec::new();
    for pass in passes.iter() {
        let up = pass[..7].replace("F", "0").replace("B", "1");
        let down = pass[7..].replace("L", "0").replace("R", "1");
        seats_id
            .push(u32::from_str_radix(&up, 2).unwrap() * 8 + u32::from_str_radix(&down, 2).unwrap())
    }
    seats_id
}

fn puzzle2(seats: &mut Vec<u32>) {
    seats.sort();
    for (i, &seat) in seats.iter().enumerate() {
        if i == seats.len() - 1 || seat + 1 != seats[i + 1] {
            println!("missing seat: {}", seat + 1);
            break;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut passes: Vec<&str> = Vec::new();
    for line in content.trim().split("\n") {
        passes.push(line);
    }

    // println!("{:?}", passes);
    let mut seats_id = puzzle1(&passes);
    println!("Valid puzzle1: {}", seats_id.iter().max().unwrap());

    puzzle2(&mut seats_id);
    // println!("Valid puzzle2: {}", valid);
}
