use petgraph::algo::bellman_ford;
// use petgraph::dot::{Config, Dot};
use petgraph::prelude::*;
use regex::Regex;
use std::env;
use std::fs;

//7:21
fn puzzle(data: &[&str]) {
    let re_fbags = Regex::new(r"^([\w\s]+) bags contain").unwrap();
    let re_bags = Regex::new(r"(\d) ([\w\s]+) bags?").unwrap();
    let mut edges: Vec<(String, String, f32)> = Vec::new();
    for line in data.iter() {
        let cap = re_fbags.captures(line).unwrap();
        let bags = re_bags.captures_iter(line);
        for cap2 in bags {
            edges.push((
                cap2[2].to_owned(),
                cap[1].to_owned(),
                cap2[1].parse().unwrap(),
            ));
            // println!("{} -> {} {} ", &cap2[2], &cap2[1], &cap[1]);
        }
    }
    let vec_ref: Vec<(&str, &str, f32)> = edges
        .iter()
        .map(|(a, b, c)| (a.as_ref(), b.as_ref(), c.to_owned()))
        .collect();
    let graphmap = DiGraphMap::<_, f32>::from_edges(&vec_ref);

    let path = bellman_ford(&graphmap, "shiny gold").unwrap();
    // println!("{:?}", path);
    println!(
        "puzzle1: {:?}",
        (path.0)
            .iter()
            .filter(|&x| x > &0.0 && x < &f32::INFINITY)
            .count()
    );
    // println!("{:?}", Dot::with_config(&graphmap, &[Config::EdgeIndexLabel]));
    
    fn recur(bag: &str, graphmap: &DiGraphMap::<&str,f32> ) -> f32 {
        let mut sum = 1f32;
        for child in graphmap.neighbors_directed(bag, Incoming) {
            let qt = graphmap.edge_weight(child, bag).unwrap();
            sum += qt*recur(child,&graphmap);
            // println!("{},{} -> {:?} : {}",ba,bag,qt,sum);
        }
        sum
    }
    println!("puzzle2: {}",recur("shiny gold", &graphmap) - 1f32);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    // let mut questions: Vec<&str> = Vec::new();
    let data: Vec<&str> = content.trim().split('\n').collect();

    puzzle(&data);
    // println!("puzzle1: {}", res);
}
