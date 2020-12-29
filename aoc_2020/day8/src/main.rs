// use regex::Regex;
use std::env;
use std::fs;

//7:24
fn genprog(data: &[&str]) -> Vec<i32> {
    let mut seen: Vec<i32> = vec![-1];
    for (i, line) in data.iter().enumerate() {
        let op: Vec<&str> = line.splitn(2, ' ').collect();
        if op[0] == "nop" || op[0] == "jmp" {
            seen.push(i as i32);
        }
    }
    seen
}

fn puzzle(data: &[&str]) {
    let possibles = genprog(&data);
    'out: for prog in possibles.iter() {
        let mut acc = 0;
        let mut pc: i32 = 0;
        let mut seen: Vec<i32> = Vec::new();
        loop {
            let mut op: Vec<&str>;
            if let Some(res) = data.get(pc as usize) {
                op = res.splitn(2, ' ').collect();
            } else {
                println!("Found good one, puz2:");
                println!("acc: {}, pc: {}", acc, pc);
                break 'out;
            }
            op[0] = if prog == &pc && op[0] == "jmp" {
                "nop"
            } else if prog == &pc && op[0] == "nop" {
                "jmp"
            } else {
                op[0]
            };
            match op[0] {
                "acc" => {
                    acc += op[1].parse::<i32>().unwrap();
                }
                "jmp" => {
                    pc = (pc as i32) - 1 + op[1].parse::<i32>().unwrap();
                }
                _ => (),
            }
            if seen.contains(&pc) {
                if prog == &-1 {
                    println!("puz1: acc: {}, pc: {}", acc, pc);
                }
                break;
            }
            seen.push(pc);
            pc += 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let data: Vec<&str> = content.trim().split('\n').collect();

    puzzle(&data);
}
