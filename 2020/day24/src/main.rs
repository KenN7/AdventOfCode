use std::collections::HashMap;
use std::env;
use std::fs;

fn puzzle(data: &Vec<&str>) {
    println!("{:?}", data);

    let mut tiles: HashMap<Vec<i32>, i32> = HashMap::new();
    for line in data.iter() {
        let mut data_chars = line.chars();
        let mut map: HashMap<String, i32> = HashMap::new();
        let mut pos: Vec<i32> = vec![0, 0];
        while let Some(c) = data_chars.next() {
            if c == 'e' {
                // println!("going {}", c);
                *map.entry(c.to_string()).or_insert(0) += 1;
                pos[0] += 2;
            } else if c == 'w' {
                *map.entry(c.to_string()).or_insert(0) += 1;
                pos[0] -= 2;
            } else if c == 's' || c == 'n' {
                let c2 = data_chars.next().unwrap();
                // println!("going {}-{}", c,c2);
                let dir = format!("{}{}", c, c2);
                *map.entry(dir.to_string()).or_insert(0) += 1;
                if dir == "se" {
                    pos[0] += 1;
                    pos[1] += -1;
                } else if dir == "sw" {
                    pos[0] += -1;
                    pos[1] += -1;
                } else if dir == "ne" {
                    pos[0] += 1;
                    pos[1] += 1;
                } else if dir == "nw" {
                    pos[0] += -1;
                    pos[1] += 1;
                }
            }
        }
        // println!("{:?}", map);
        println!("{:?}", pos);
        *tiles.entry(pos.clone()).or_insert(0) += 1;
    }
    println!("{:?}", tiles);
    println!(
        "Number of flipped {}",
        tiles.values().filter(|&&x| x % 2 == 1).count()
    );

    let mut new_map = tiles.clone();
    let mut map = tiles.clone();

    for t in 0..100 {
        for (point, c) in map.iter() {
            let active = get_neighbors(&point, &map);
            if c % 2 == 1 && (active == 0 || active > 2) {
                *new_map.entry(point.clone()).or_insert(0) += 1;
            } else if c % 2 == 0 && active == 2 {
                *new_map.entry(point.clone()).or_insert(0) += 1;
            }
        }
        let new_points = gen_new_actives(&map);
        for new_act in new_points.iter() {
            *new_map.entry(new_act.clone()).or_insert(0) += 1;
        }
        // map = new_map.clone();
        map = new_map.iter().filter(|(_k,&v)| v%2 == 1 ).map(|x| (x.0.to_vec(),*x.1) ).collect();

        // println!("new p: {:?}", new_points);
        // println!("points :{:?}", map);
        println!(
            "day {} - Number of flipped {}",t+1,
            map.values().filter(|&&x| x % 2 == 1).count()
        );
    }
}

fn gen_new_actives(map: &HashMap<Vec<i32>, i32>) -> Vec<Vec<i32>> {
    let mut new_points = HashMap::new();
    for pos in map.iter().filter(|(_x, &c)| c % 2 == 1).map(|(x, _c)| x) {
        for p in [[1, 1], [-1, 1], [-2, 0], [2, 0], [1, -1], [-1, -1]].iter() {
            let point: Vec<i32> = p.iter().enumerate().map(|(i, x)| pos[i] + x).collect();
             if map.contains_key(&point.to_vec()) {
                 continue;
             }
            *new_points.entry(point).or_insert(0) += 1;
        }
    }
    new_points
        .iter()
        .filter(|(_x, &n)| n == 2)
        .map(|(x, _n)| x.clone())
        .collect()
}

fn get_neighbors(pos: &Vec<i32>, map: &HashMap<Vec<i32>, i32>) -> i32 {
    let mut count = 0;
    for p in [[1, 1], [-1, 1], [-2, 0], [2, 0], [1, -1], [-1, -1]].iter() {
        let point: Vec<i32> = p.iter().enumerate().map(|(i, x)| pos[i] + x).collect();
        if let Some(c) = map.get(&point) {
            if c % 2 == 1 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = content.trim().split('\n').collect();

    puzzle(&data);
}
