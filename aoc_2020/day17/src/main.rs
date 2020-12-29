use itertools::Itertools;
use std::collections::BTreeMap;
use std::env;
use std::fs;

fn pprint(data: &BTreeMap<Vec<i32>, char>) {
    let (y_min, y_max) = (
        data.keys().min_by_key(|x| x[1]).unwrap()[1],
        data.keys().max_by_key(|x| x[1]).unwrap()[1],
    );
    let (z_min, z_max) = (
        data.keys().min_by_key(|x| x[2]).unwrap()[2],
        data.keys().max_by_key(|x| x[2]).unwrap()[2],
    );

    for z in z_min..z_max + 1 {
        println!("z={}", z);
        let pz = data
            .keys()
            .filter(|x| x[2] == z)
            .collect::<Vec<&Vec<i32>>>();
        // print!("pz: {:?}", pz);
        for y in y_min..y_max + 1 {
            let p2 = pz.iter().filter(|i| i[1] == y);
            // print!("p2: {:?}", p2);
            for k in p2.sorted_by_key(|x| x[0]) {
                print!("{}", data[*k]);
                // print!("{:?}", k);
            }
            println!();
        }
        println!();
    }
}

fn get_neighbors(pos: &Vec<i32>, map: &BTreeMap<Vec<i32>, char>) -> i32 {
    let mut count = 0;
    for p in (0..pos.len()).map(|_i| -1..2).multi_cartesian_product() {
        let point: Vec<i32> = p.iter().enumerate().map(|(i, x)| pos[i] - x).collect();
        if point == *pos {
            continue;
        }
        if let Some(c) = map.get(&point) {
            if c == &'#' {
                count += 1;
            }
        }
    }
    count
}

fn gen_new_actives(map: &BTreeMap<Vec<i32>, char>) -> Vec<Vec<i32>> {
    let mut new_points = BTreeMap::new();
    for pos in map.iter().filter(|(_x, &c)| c == '#').map(|(x, _c)| x) {
        for p in (0..pos.len()).map(|_i| -1..2).multi_cartesian_product() {
            let point: Vec<i32> = p.iter().enumerate().map(|(i, x)| pos[i] - x).collect();
            if point == *pos {
                continue;
            }
            *new_points.entry(point).or_insert(0) += 1;
        }
    }
    new_points
        .iter()
        .filter(|(_x, &n)| n == 3)
        .map(|(x, _n)| x.clone())
        .collect()
}

fn puzzle(data: &BTreeMap<Vec<i32>, char>) {
    let mut new_map = data.clone();
    let mut map = data.clone();

    for _t in 0..6 {
        for (point, &c) in map.iter() {
            let active = get_neighbors(&point, &map);
            if c == '#' && active != 2 && active != 3 {
                new_map.insert(point.clone(), '.');
            } else if c == '.' && active == 3 {
                new_map.insert(point.clone(), '#');
            }
        }
        new_map.extend(gen_new_actives(&map).iter().map(|x| (x.clone(), '#')));
        map = new_map.clone();
    }

    println!("{}", map.values().filter(|&&x| x == '#').count());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut map = BTreeMap::new();
    let mut map4d = BTreeMap::new();
    let data: Vec<&str> = content.trim().split('\n').collect();
    for (y, line) in data.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            map.insert(vec![x as i32, y as i32, 0 as i32], ch);
            map4d.insert(vec![x as i32, y as i32, 0 as i32, 0 as i32], ch);
        }
    }

    pprint(&map);
    puzzle(&map);
    puzzle(&map4d);
}
