use std::env;
use std::fs;

//8:44

fn puzzle1(time: i64, buses: &[(i64, i64)]) {
    let times: Vec<(i64, i64)> = buses
        .iter()
        .map(|x| (((time / x.0) + 1) * x.0, x.0))
        .collect();
    println!("{:?}", times.iter().min().unwrap());
    println!(
        "{:?}",
        (times.iter().min().unwrap().0 - time) * times.iter().min().unwrap().1
    )
}

fn puzzle2(buses: &[(i64, i64)]) {
    let mut count: i64 = 0;
    let mut add = 1;
    for bus in buses.iter() {
        while count.rem_euclid(bus.0) != (bus.0 - bus.1).rem_euclid(bus.0) {
            count += add;
        }
        add *= bus.0;
    }
    println!("count: {:?}", count);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = content.trim().split('\n').collect();

    let time: i64 = data[0].parse().unwrap();
    let buses: Vec<(i64, i64)> = data[1]
        .split(',')
        .enumerate()
        .map(|(i, x)| (x.parse().unwrap_or(-1), i as i64))
        .filter(|x| x.0 != -1)
        .collect();

    println!("{:?} {:?}", time, buses);
    puzzle1(time, &buses);
    puzzle2(&buses);
}
