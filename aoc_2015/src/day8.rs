use crate::input;
use std::iter::FromIterator;
// use std::collections::HashMap;

pub fn day8() -> input::Result<()> {
    let contents = input::load_day_file("day8.txt");

    let mut count = 0;
    let mut count2 = 0;
    for lit in contents.lines() {
        let char_num = lit.len();
        let unescaped = unescape(&lit[1..lit.len() - 1]);
        let char_mem = unescaped.len();
        count += char_num - char_mem;
        let escaped = escape(lit);
        let char_escaped = escaped.len();
        count2 += char_escaped - char_num;

        // println!("{} <- {} -> {}", escaped, lit, unescaped);
        // println!("{} - {} = {}", char_num, char_mem, char_num - char_mem);
    }

    println!("Count with unescape 📜: {}", count);
    println!("Count after escape 📜: {}", count2);

    Ok(())
}

fn escape(s: &str) -> String {
    let mut escaped = String::with_capacity(s.len());
    escaped.push('"');
    for c in s.chars() {
        if c == '"' {
            escaped += "\\\"";
        } else if c == '\\' {
            escaped += "\\\\";
        } else {
            escaped.push(c);
        }
    }
    escaped.push('"');
    escaped
}

fn unescape(s: &str) -> String {
    let mut unescaped = String::new();
    let mut it = s.chars();
    while let Some(c) = it.next() {
        match c {
            '\\' => match it.next() {
                Some('x') => {
                    let num = u8::from_str_radix(
                        String::from_iter(&[it.next().unwrap(), it.next().unwrap()]).as_str(),
                        16,
                    )
                    .unwrap();
                    // escaped.push_str((num as char).to_string().as_str());
                    unescaped.push_str("0".to_string().as_str()); //for some reason, white chars produce extra len?
                }
                Some(c2) => unescaped.push_str(c2.to_string().as_str()),
                None => (),
            },
            _ => {
                unescaped.push_str(c.to_string().as_str());
            }
        }
    }

    unescaped
}
