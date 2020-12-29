use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<Vec<&str>> = content
        .trim()
        .split("\n")
        .map(|x| x.split(" (contains ").collect())
        .collect();
    let food: Vec<(Vec<String>, Vec<String>)> = data
        .iter()
        .map(|x| {
            (
                x[0].split(' ').map(|x| x.to_string()).collect(),
                x[1].replace(")", "")
                    .split(", ")
                    .map(|x| x.to_string())
                    .collect(),
            )
        })
        .collect();
    println!("food: {:?}", food);

    let mut allerg_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for (foods, allergs) in food.iter() {
        for allerg in allergs.iter() {
            match allerg_map.get(allerg.as_str()) {
                Some(set) => {
                    let set2: HashSet<&str> = foods.iter().map(|x| x.as_str()).collect();
                    let new_set = set & &set2;
                    let set4 = allerg_map.get_mut(allerg.as_str()).unwrap();
                    *set4 = new_set;
                }
                None => {
                    let set2: HashSet<&str> = foods.iter().map(|x| x.as_str()).collect();
                    allerg_map.insert(allerg.as_str(), set2);
                }
            }
        }
    }
    println!("am: {:?}", allerg_map);

    let bad_food: HashSet<&&str> = allerg_map.values().flatten().collect();
    println!("bad set: {:?}", bad_food);

    let good_food: Vec<&str> = food
        .iter()
        .map(|x| x.0.iter().map(|x| x.as_str()))
        .flatten()
        .filter(|x| !bad_food.contains(x))
        .collect();
    // println!("Yummy: {:?}", good_food);
    println!("Yummy Count: {:?}", good_food.len());

    let mut exact_map: HashMap<&str,&str> = HashMap::new();
    while allerg_map.values().flatten().count() > 0 {
        let mut food_to_rm = "";
        for allerg in allerg_map.iter_mut() {
            println!("{:?}", allerg);
            if allerg.1.len() == 1 {
                food_to_rm = allerg.1.iter().nth(0).unwrap().clone();
                exact_map.insert(food_to_rm, allerg.0);
                break;
                // allerg_map.get_mut(allerg.0).unwrap().remove(food);
            }
        }
        for allerg in allerg_map.iter_mut() {
            allerg.1.remove(food_to_rm);
        }
        println!("{:?}",allerg_map.values().flatten().collect::<Vec<&&str>>())
    }
    println!("exact: {:?}", exact_map);

    println!("{:?}", exact_map.iter().sorted_by_key(|x| x.1).map(|x| x.0).format(",").to_string().replace("\"","") )
}
