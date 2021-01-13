use crate::input;
// use ndarray::Array2;
// use onig::Regex;
// use std::collections::HashMap;
use serde_json::{Result, Value};

pub fn day12() -> input::Result<()> {
    let contents = input::load_day_file("day12.txt");

    let v: Value = serde_json::from_str(contents.as_str())?;

    fn recur(v: &Value, p1: bool) -> i64 {
        let mut ret = 0;
        if v.is_array() {
            for a in v.as_array().unwrap().iter() {
                ret += recur(a, p1);
            }
        } else if v.is_object() {
            if p1
                || !v
                    .as_object()
                    .unwrap()
                    .values()
                    .any(|x| x == &Value::String("red".to_string()))
            {
                for a in v.as_object().unwrap().values() {
                    ret += recur(a, p1);
                }
            }
        } else if v.is_u64() {
            ret += v.as_i64().unwrap();
        } else if v.is_i64() {
            ret += v.as_i64().unwrap();
        } else if v.is_f64() {
            ret += v.as_f64().unwrap() as i64;
        }
        ret
    }

    let val = recur(&v, true);
    println!("🧝 Sum of numbers 🔢 is {:?}", val);
    let val = recur(&v, false);
    println!("🤓 Sum of 🔴 non-red numbers is {:?}", val);

    Ok(())
}
