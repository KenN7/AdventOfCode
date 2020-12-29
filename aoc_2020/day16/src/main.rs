use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

fn puzzle1(classes: &HashMap<&str, Vec<i32>>, tickets: &Vec<Vec<i32>>, my: &Vec<i32>) {
    let flat_classes = classes.values().flatten().collect::<Vec<&i32>>();
    let invalid: Vec<&i32> = tickets
        .iter()
        .flatten()
        .filter(|&x| !flat_classes.contains(&x))
        .collect();

    println!("Puzzle 1: {}", invalid.iter().copied().sum::<i32>());

    //--------------- Part 2 -----------------

    let valid: Vec<&Vec<i32>> = tickets
        .iter()
        .filter(|&x| x.iter().all(|y| flat_classes.contains(&y)))
        .collect();

    // Classical way:
    // let mut possible = HashMap::new();
    // for icol in 0..valid[0].len() {
    //     let col: Vec<i32> = valid.iter().map(|x| x[icol]).collect();
    //     // println!("col: {:?}", col);
    //     for class in classes.keys() {
    //         if col.iter().all(|x| classes[*class].contains(x)) {
    //             // println!("field {} is ok for col {}", class, icol);
    //             possible.entry(class).or_insert_with(Vec::new).push(icol);
    //         }
    //     }
    // }

    // More functionnal way:
    let mut possible: HashMap<&str, Vec<usize>> = classes
        .keys()
        .map(|&x| {
            (
                x,
                (0..classes.iter().len())
                    .filter(|&i| {
                        valid
                            .iter()
                            .map(|col| col[i])
                            .all(|v| classes[x].contains(&v))
                    })
                    .collect(),
            )
        })
        .collect();

    // println!("possibles: {:?}", possible);

    let mut all_empty = possible.iter().all(|(_i, x)| x.len() == 0);
    let mut fields = Vec::with_capacity(classes.keys().len());
    fields.resize(classes.keys().len(), "");
    while !all_empty {
        let candidate = possible
            .iter()
            .filter(|(_i, x)| x.len() == 1)
            .nth(0)
            .unwrap();
        let class = candidate.0;
        let id = candidate.1[0];
        fields[id] = class; //.to_string();
        for classes in possible.values_mut() {
            classes.retain(|&x| x != id);
        }
        all_empty = possible.iter().all(|(_i, x)| x.len() == 0);
        // println!("nb allemp: {}", all_empty);
    }

    let my_t_map: HashMap<&str, &i32> = fields.iter().copied().zip(my.iter()).collect();
    println!("{:?}", my_t_map);

    println!(
        "Puzzle2 prod: {}",
        my_t_map
            .iter()
            .filter(|x| x.0.starts_with("departure"))
            .map(|(_i,&&x)| x as i64)
            .product::<i64>()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = content.trim().split("\n\n").collect();

    let classes: Vec<Vec<&str>> = data
        .get(0)
        .unwrap()
        .split('\n')
        .map(|l| l.split(':').collect())
        .collect();
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut map_classes = HashMap::new();
    for class in classes.iter() {
        let mut range: Vec<i32> = Vec::new();
        for cap in re.captures_iter(class[1]) {
            range.extend(cap[1].parse::<i32>().unwrap()..cap[2].parse::<i32>().unwrap() + 1);
        }
        map_classes.insert(class[0], range);
    }

    let your_t: Vec<i32> = data
        .get(1)
        .unwrap()
        .split('\n')
        .skip(1)
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .flatten()
        .collect();

    let nb_t: Vec<Vec<i32>> = data
        .get(2)
        .unwrap()
        .split('\n')
        .skip(1)
        .map(|x| x.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    // println!("{:?}", map_classes);
    // println!("{:?}", your_t);
    // println!("{:?}", nb_t);

    puzzle1(&map_classes, &nb_t, &your_t);
}
