use std::fs;
use std::path::PathBuf;
use std::time::Instant;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Day {
    name: String,
    func: Option<fn() -> Result<()>>,
}

impl Day {
    pub fn new(name: &str, func: fn() -> Result<()>) -> Self {
        Day {
            name: name.to_string(),
            func: Some(func),
        }
    }

    pub fn run(&self) {
        println!("------ {} ------", self.name);
        let start = Instant::now();
        match (self.func.unwrap())() {
            Ok(_) => (),
            Err(e) => panic!("⚠️ Error: {}, executing {}", e, self.name),
        }
        let duration = start.elapsed();
        println!();
        println!("⏱️  of exec: {:?}", duration);
        println!();
    }
}

pub fn load_day_file(filename: &str) -> String {
    let path: PathBuf = ["data/", filename].iter().collect();
    fs::read_to_string(path).expect("Something went wrong reading the file")
}
