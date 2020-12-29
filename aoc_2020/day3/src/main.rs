use std::env;
use std::fs;

fn puzzle1(forest: &Vec<char>, width: usize, height: usize, right: usize, down: usize) -> u32 {
    let mut init = (0, 0);
    let mut count = 0;
    while init.1 < height {
        if forest[init.0 % width + width * init.1] == '#' {
            // println!("tree at {}", y);
            count += 1;
        }
        init.0 += right;
        init.1 += down;
    }
    println!("Counter: {}, for r{},d{}", count, right, down);
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let height = content.lines().count();
    let width = (content.lines().next().unwrap()).chars().count();
    println!("h:{}, w:{}", height, width);
    let mut forest: Vec<char> = vec!['.'; width * height];

    for (y, line) in content.lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == '#' {
                forest[x + y * width] = '#';
            }
        }
    }

    // println!("{:?}", forest);

    puzzle1(&forest, width, height, 3, 1);

    println!(
        "product: {}",
        puzzle1(&forest, width, height, 1, 1)
            * puzzle1(&forest, width, height, 3, 1)
            * puzzle1(&forest, width, height, 5, 1)
            * puzzle1(&forest, width, height, 7, 1)
            * puzzle1(&forest, width, height, 1, 2)
    )
}
