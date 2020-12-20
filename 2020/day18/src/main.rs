// use itertools::Itertools;
// use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::env;
use std::fs;

fn simple_op(calc: &mut VecDeque<String>) -> i64 {
    let mut res: i64 = i64::from_str_radix(&calc.pop_back().unwrap(), 10).unwrap();
    while let Some(chr) = calc.pop_back() {
        match chr.as_ref() {
            "+" => {
                res += i64::from_str_radix(&calc.pop_back().unwrap(), 10).unwrap();
            }
            "*" => {
                res *= i64::from_str_radix(&calc.pop_back().unwrap(), 10).unwrap();
            }
            ")" => {
                break;
            }
            _ => (),
        }
    }
    res
}

fn op_prio(calc: &mut VecDeque<String>) -> i64 {
    let mut add_done: VecDeque<String> = VecDeque::new();
    println!("subcal: {:?}", calc);
    let mut res: i64 = i64::from_str_radix(&calc.pop_back().unwrap(), 10).unwrap();
    loop {
        match &calc.pop_back() {
            Some(v) => match v.as_ref() {
                "+" => {
                    res += i64::from_str_radix(&calc.pop_back().unwrap(), 10).unwrap();
                }
                "*" => {
                    add_done.push_front(res.to_string());
                    res = i64::from_str_radix(&calc.pop_back().unwrap(), 10).unwrap();
                    add_done.push_front(v.to_string());
                }
                ")" => {
                    add_done.push_front(res.to_string());
                    break;
                }
                _ => {
                    add_done.push_front(v.to_string());
                }
            },
            None => {
                add_done.push_front(res.to_string());
                break;
            }
        }
    }
    println!("add done: {:?}", add_done);

    let mut res: i64 = i64::from_str_radix(&add_done.pop_back().unwrap(), 10).unwrap();
    while add_done.len() > 0 {
        let chr = add_done.pop_back().unwrap();
        match chr.as_ref() {
            "*" => {
                res *= i64::from_str_radix(&add_done.pop_back().unwrap(), 10).unwrap();
            }
            ")" => {
                break;
            }
            _ => (),
        }
    }
    res
}

fn puzzle(data: &Vec<String>, op: &dyn Fn(&mut VecDeque<String>) -> i64) {
    let mut results: Vec<i64> = Vec::new();
    for calc in data.iter() {
        let mut sub_cal: VecDeque<String> = VecDeque::new();
        for chr in calc.chars().rev() {
            match chr {
                '(' => {
                    let res = op(&mut sub_cal).to_string();
                    sub_cal.push_back(res);
                }
                _ => {
                    let s = chr.to_string();
                    sub_cal.push_back(s);
                }
            }
        }
        let number = op(&mut sub_cal);
        println!("finished: {}", number);
        results.push(number);
    }

    println!("Sum Puzzle : {}", results.iter().sum::<i64>());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<String> = content
        .trim()
        .split('\n')
        .map(|x| x.replace(" ", ""))
        .collect();

    println!("{:?}", data);
    puzzle(&data, &simple_op);
    puzzle(&data, &op_prio);
}
