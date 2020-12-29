use std::collections::HashSet;
use std::env;
use std::fs;

fn puzzle1(questions: &Vec<&str>) -> u32 {
    let mut nb_yes = 0;
    for q in questions {
        nb_yes += q.replace("\n", "").chars().collect::<HashSet<char>>().len() as u32;
    }
    nb_yes
}

fn puzzle2(questions: &Vec<&str>) -> u32 {
    let mut nb_yes = 0;
    for q in questions {
        let vecset = q
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.chars().collect::<HashSet<char>>())
            .collect::<Vec<HashSet<char>>>();
        let inter = vecset.iter().fold(vecset[0].clone(), |set1, set2| {
            set1.intersection(set2).cloned().collect()
        });
        nb_yes += inter.len() as u32;
    }
    // for q in questions {
    //     // let mut a = HashMap::new();
    //     let mut b = HashMap::new();
    //     for group in q.lines() {
    //         for ans in group.chars() {
    //             b.insert(ans, b.get(&ans).unwrap_or(&0) + 1);
    //         }
    //     }
    //     for (_key, &val) in b.iter() {
    //         if val == q.lines().count() {
    //             nb_yes += 1
    //         }
    //     }
    // }
    nb_yes
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut questions: Vec<&str> = Vec::new();
    for line in content.trim().split("\n\n") {
        questions.push(line);
    }

    // println!("{:?}", passes);
    let res = puzzle1(&questions);
    println!("puzzle1: {}", res);

    let res = puzzle2(&questions);
    println!("puzzle2: {}", res);
}
