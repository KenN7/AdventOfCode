use std::fs;
use std::path::PathBuf;

pub struct Day {
    name: String,
    func: Option<fn()>,
}

impl Day {
    pub fn new(name: &str, func: fn()) -> Self {
        Day {
            name: name.to_string(),
            func: Some(func),
        }
    }

    pub fn run(&self) {
        println!("------ {} ------", self.name);
        (self.func.unwrap())();
        println!()
    }
}

pub fn load_day_file(filename: &str) -> String {
    let path: PathBuf = ["data/", filename].iter().collect();
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    contents
}
