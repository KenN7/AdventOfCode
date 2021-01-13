use crate::input;
use onig::Regex;
use std::fmt;
use std::iter::FromIterator;

struct PwdGenerator {
    chars: Vec<char>,
}

impl PwdGenerator {
    fn new(s: &str) -> Self {
        PwdGenerator {
            chars: s.chars().collect(),
        }
    }

    fn next(&mut self) {
        let mut len = self.chars.len() - 1;
        let mut c = self.chars[len];
        while c == 'z' {
            self.chars[len] = 'a';
            if len == 0 {
                self.chars.insert(0, (b'a' as u8 - 1) as char);
                break;
            }
            len -= 1;
            c = self.chars[len]
        }
        self.chars[len] = (self.chars[len] as u8 + 1) as char;
    }
}

impl fmt::Display for PwdGenerator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.chars
                .iter()
                .fold(String::new(), |acc, e| acc + &e.to_string())
        )
    }
}

fn is_pwd_valid(s: &str, re: &Regex, abc: &Vec<String>) -> bool {
    if ["i", "o", "l"].iter().any(|e| s.contains(e)) {
        return false;
    } else if abc.iter().all(|e| !s.contains(e)) {
        return false;
    } else if re.find(s).is_none() {
        return false;
    }
    true
}

pub fn day11() -> input::Result<()> {
    let contents = input::load_day_file("day11.txt");

    let mut pass = PwdGenerator::new(contents.trim());

    let re_d = Regex::new(r"([a-z])\1.*([a-z])\2").unwrap();
    let abc: Vec<_> = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .collect::<Vec<_>>()
        .windows(3)
        .map(|x| String::from_iter(x.iter()))
        .collect();

    let mut found = false;
    while !found {
        pass.next();
        found = is_pwd_valid(pass.to_string().as_str(), &re_d, &abc);
    }

    println!("Found 🎅 next 🔑 password: {}", pass);

    found = false;
    while !found {
        pass.next();
        found = is_pwd_valid(pass.to_string().as_str(), &re_d, &abc);
    }

    println!("Found yet another 🔑: {}", pass);

    Ok(())
}
