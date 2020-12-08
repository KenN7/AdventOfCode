use std::env;
use std::fs;

fn puzzle2(numbers: &Vec<i32>) {
    for number1 in numbers.iter() {
        for number2 in numbers.iter() {
            for number3 in numbers.iter() {
                if number1 + number2 + number3 == 2020 {
                    println!(
                        "{} * {} * {} = {}",
                        number1,
                        number2,
                        number3,
                        number3 * number1 * number2
                    )
                }
            }
        }
    }
}

fn puzzle1(numbers: &Vec<i32>) {
    for number1 in numbers.iter() {
        for number2 in numbers.iter() {
            if number1 + number2 == 2020 {
                println!("{} * {} = {}", number1, number2, *number1 * number2)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut results: Vec<i32> = Vec::new();
    for line in contents.lines() {
        results.push(line.parse().unwrap());
    }
    //println!("{:?}", results);

    puzzle1(&results);
    puzzle2(&results);
}

// ________________________________________________________
// Executed in   33,33 secs   fish           external 
//    usr time   32,83 secs  124,88 millis   32,71 secs 
//    sys time    0,20 secs   55,06 millis    0,15 secs 

