use std::env;
use std::fs;
use std::collections::HashMap;

pub fn day1() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut map = HashMap::new(); 
    for (i,c) in contents.trim().chars().enumerate() {
        *map.entry(c).or_insert(0) += 1;
        
        if map.get(&'(').unwrap_or(&0) - map.get(&')').unwrap_or(&0) == -1 {
            println!("Basement at position: {} ", i+1);
        }
    }
    println!("Floor 🏠: {} ", map.get(&'(').unwrap() - map.get(&')').unwrap());
}
