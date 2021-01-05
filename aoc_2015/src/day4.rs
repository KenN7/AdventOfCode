use crate::input;
use md5::{Digest, Md5};

pub fn day4() -> input::Result<()> {
    let contents = String::from("bgvyzdsv");

    let mut i = 1;
    let mut hash;
    let mut part1 = false;
    let mut part2 = false;
    loop {
        hash = Md5::digest(format!("{}{}", contents, i).as_bytes());
        let text_hash = format!("{:x}", hash);
        if text_hash.starts_with("00000") && part1 == false {
            println!("Part1: 🔑 {} and hash is {}", i, text_hash);
            part1 = true;
        }
        if text_hash.starts_with("000000") && part2 == false {
            println!("Part2: 🔑 {} and hash is {}", i, text_hash);
            part2 = true;
        }
        if part1 && part2 {
            break;
        }
        i += 1;
    }
    Ok(())
}
