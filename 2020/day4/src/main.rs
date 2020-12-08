use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

fn puzzle1(passports: &Vec<&str>) -> u32 {
    let re = Regex::new(r"(\w*):(\S*)").unwrap();
    let mut valid = 0;
    let mut fields: Vec<String> = Vec::new();
    for passport in passports.iter() {
        // println!("Next passport:");
        fields.clear();
        for cap in re.captures_iter(passport) {
            // println!("{:?}",cap);
            fields.push(cap[1].to_owned());
        }
        if fields.iter().any(|e| e == "byr")
            && fields.iter().any(|e| e == "iyr")
            && fields.iter().any(|e| e == "eyr")
            && fields.iter().any(|e| e == "hgt")
            && fields.iter().any(|e| e == "hcl")
            && fields.iter().any(|e| e == "ecl")
            && fields.iter().any(|e| e == "pid")
        {
            valid += 1;
        }
    }
    valid
}

fn puzzle2(passports: &Vec<&str>) -> u32 {
    let re = Regex::new(r"(\w*):(\S*)").unwrap();
    let mut valid = 0;
    let mut fields = HashMap::new();
    for (_i,passport) in passports.iter().enumerate() {
        // println!("Next passport:");
        fields.clear();
        for cap in re.captures_iter(passport) {
            // println!("{:?}",cap);
            fields.insert(cap[1].to_owned(), cap[2].to_owned());
        }
        if fields.keys().any(|e| e == "byr")
            && fields.keys().any(|e| e == "iyr")
            && fields.keys().any(|e| e == "eyr")
            && fields.keys().any(|e| e == "hgt")
            && fields.keys().any(|e| e == "hcl")
            && fields.keys().any(|e| e == "ecl")
            && fields.keys().any(|e| e == "pid")
        {
            let len = fields.get("hgt").unwrap().len() - 2;
            // println!("{}",fields.get("byr").unwrap());
            if 1920 <= fields.get("byr").unwrap().parse().unwrap()
                && 2002 >= fields.get("byr").unwrap().parse().unwrap()
                && 2010 <= fields.get("iyr").unwrap().parse().unwrap()
                && 2020 >= fields.get("iyr").unwrap().parse().unwrap()
                && 2020 <= fields.get("eyr").unwrap().parse().unwrap()
                && 2030 >= fields.get("eyr").unwrap().parse().unwrap()
                && fields.get("pid").unwrap().len() == 9
                && fields.get("hcl").unwrap().len() == 7
                && ((fields.get("hgt").unwrap().ends_with("cm")
                    && 150 <= fields.get("hgt").unwrap()[..len].parse().unwrap()
                    && 193 >= fields.get("hgt").unwrap()[..len].parse().unwrap())
                    || (fields.get("hgt").unwrap().ends_with("in")
                        && 59 <= fields.get("hgt").unwrap()[..len].parse().unwrap()
                        && 76 >= fields.get("hgt").unwrap()[..len].parse().unwrap()))
                && (fields.get("ecl").unwrap().contains("amb")
                    || fields.get("ecl").unwrap().contains("blu")
                    || fields.get("ecl").unwrap().contains("brn")
                    || fields.get("ecl").unwrap().contains("gry")
                    || fields.get("ecl").unwrap().contains("grn")
                    || fields.get("ecl").unwrap().contains("hzl")
                    || fields.get("ecl").unwrap().contains("oth"))
            {
                valid += 1;
            }
        }
    }
    valid
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut passports: Vec<&str> = Vec::new();
    for line in content.split("\n\n") {
        passports.push(line);
    }

    // println!("{:?}", passports);
    let valid = puzzle1(&passports);
    println!("Valid puzzle1: {}", valid);
    let valid = puzzle2(&passports);
    println!("Valid puzzle2: {}", valid);
}
