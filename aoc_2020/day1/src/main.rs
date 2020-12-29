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

    let results: Vec<i32> = contents.lines().map(|x| x.parse().unwrap()).collect();
    //println!("{:?}", results);

    puzzle1(&results);
    puzzle2(&results);
}

// ________________________________________________________
//      Executed in    8,08 secs   fish           external 
//         usr time    7,38 secs   53,27 millis    7,33 secs 
//         sys time    0,69 secs   38,52 millis    0,65 secs 


