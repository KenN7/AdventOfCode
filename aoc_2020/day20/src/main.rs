use ndarray::prelude::*;
use ndarray::{arr2, concatenate, Axis};
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug)]
struct Tile {
    id: i32,
    data: Array2<u32>,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            id: 0,
            data: Array2::<u32>::zeros((10, 10)),
        }
    }

    fn rotate90(&mut self) {
        self.data.swap_axes(0, 1);
        self.data.invert_axis(Axis(1));
    }

    fn flip(&mut self, axes: usize) {
        self.data.invert_axis(Axis(axes));
    }

    fn get_edges(&self) -> Vec<Array1<u32>> {
        let mut edges: Vec<Array1<u32>> = Vec::new();
        edges.push(self.data.row(0).to_owned());
        edges.push(self.data.row(9).to_owned());
        edges.push(self.data.column(0).to_owned());
        edges.push(self.data.column(9).to_owned());
        edges
    }

    fn move_until<F: Fn(&Tile) -> bool>(&mut self, test: F) {
        for flip in 0..3 {
            if flip == 1 {
                self.flip(0);
            } else if flip == 2 {
                self.flip(1);
            }
            for _rot in 0..4 {
                if test(&self) {
                    // println!("Rotated till sucess! ",);
                    return;
                }
                self.rotate90();
            }
        }
    }
}

fn parse_tile(input: &[&str]) -> Tile {
    let mut tile = Tile::new();
    tile.id = input[0]
        .split(' ')
        .nth(1)
        .unwrap()
        .replace(":", "")
        .parse()
        .unwrap();
    for (ix, line) in input.iter().skip(1).enumerate() {
        //println!("Tile {}, line = {}", id, line);
        for (col, data) in line.chars().enumerate() {
            tile.data[[ix, col]] = (data == '#') as u32;
        }
    }
    //println!("tile = {:?}", tile);
    tile
}

fn puzzle1(tiles: &mut Vec<Tile>) {
    let mut all_edges: HashMap<Array1<u32>, (i32, Vec<i32>)> = HashMap::new();
    for t in tiles.iter() {
        let edges_1 = t.get_edges();
        let edges_rev: Vec<Array1<u32>> = t
            .get_edges()
            .iter()
            .map(|x| x.slice(s![..;-1]).to_owned())
            .collect();
        for ed in edges_1.iter().chain(edges_rev.iter()) {
            let entry = all_edges.entry(ed.to_owned()).or_insert((0, Vec::new()));
            entry.0 += 1;
            entry.1.push(t.id);
        }
    }
    // println!("Hashmap : {:?}", all_edges);

    let mut corners: HashMap<i32, i32> = HashMap::new();
    all_edges
        .iter()
        .filter(|(_edge, data)| data.0 == 1)
        .map(|(_edge, data)| data.1[0])
        .for_each(|x| {
            *corners.entry(x).or_insert(0) += 1;
        });

    let res: i64 = corners
        .iter()
        .filter(|(_id, val)| **val == 4)
        .map(|(&i, _x)| i as i64)
        .product();

    // println!("edges tiles : {:?}", edges_tiles);
    println!("corners : {:?}", corners);
    println!("Result Part1: {}", res);

    //select one corner and get good edges
    let first_corner: i32 = corners
        .iter()
        .filter(|(_id, val)| **val == 4)
        .map(|(&i, _x)| i)
        .next()
        .unwrap();
    let edges_tiles_vals: HashMap<Array1<u32>, (i32, Vec<i32>)> = all_edges
        .iter()
        .filter(|(_edge, data)| data.1[0] == first_corner && data.0 == 1)
        .map(|(x, y)| (x.clone(), y.clone()))
        .collect();
    // println!("edges tiles : {:?}", edges_tiles_vals);
    println!("first corner : {}", first_corner);

    let size_f: usize = (tiles.iter().len() as f64).sqrt() as usize;

    let mut tiles_map: HashMap<i32, &mut Tile> = tiles.iter_mut().map(|x| (x.id, x)).collect();

    let first_tile = tiles_map.get_mut(&first_corner).unwrap();

    let mut good_edges: Vec<Array1<u32>> = Vec::new();
    for edge in edges_tiles_vals.keys() {
        if first_tile.get_edges().iter().any(|x| x == edge) {
            good_edges.push(edge.clone())
        }
    }
    // flip and rotate until edges are on top left for first tile

    first_tile.move_until(|x| {
        good_edges[0] == x.data.row(0) && good_edges[1] == x.data.column(0)
            || good_edges[0].slice(s![..;-1]) == x.data.row(0) && good_edges[1] == x.data.column(0)
            || good_edges[0] == x.data.row(0) && good_edges[1].slice(s![..;-1]) == x.data.column(0)
    });

    let mut final_place: Array2<i32> = Array2::<i32>::zeros((size_f, size_f));
    final_place[[0, 0]] = first_tile.id;

    // construct list of neighboring tiles by rotating and flipping connected tiles
    for j in 0..size_f {
        for i in 1..size_f {
            let tile = tiles_map.get(&final_place[[j, i - 1]]).unwrap();
            let next = all_edges
                .get(&tile.data.column(9).to_owned())
                .unwrap()
                .1
                .iter()
                .find(|&&x| x != tile.id)
                .unwrap();
            // println!("next: {}", next);
            final_place[[j, i]] = *next;

            // println!("j:{},i:{}", j, i);
            let wanted = tile.data.column(9).to_owned();
            let next_tile = tiles_map.get_mut(&next).unwrap();
            next_tile.move_until(|x| wanted == x.data.column(0))
        }
        if j != size_f - 1 {
            let tile = tiles_map.get(&final_place[[j, 0]]).unwrap();
            let next = all_edges
                .get(&tile.data.row(9).to_owned())
                .unwrap()
                .1
                .iter()
                .find(|&&x| x != tile.id)
                .unwrap();
            // println!("next: {}", next);
            final_place[[j + 1, 0]] = *next;

            let wanted = tile.data.row(9).to_owned();
            let next_tile = tiles_map.get_mut(&next).unwrap();
            next_tile.move_until(|x| wanted == x.data.row(0))
        }
    }
    println!("final p:\n{:?}", final_place);

    //construct full image by concatenation of cropped data from flipped and rotated arrays
    let mut full_pic: Array2<u32> = arr2(&[[]]);
    for (i, row) in final_place.genrows().into_iter().enumerate() {
        let mut line_pic = tiles_map
            .get(&final_place[[i, 0]])
            .unwrap()
            .data
            .slice(s![1..9, 1..9])
            .to_owned();
        for (j, sub) in row.iter().enumerate() {
            if j != 0 {
                line_pic = concatenate!(
                    Axis(1),
                    line_pic,
                    tiles_map.get(&sub).unwrap().data.slice(s![1..9, 1..9])
                );
            }
        }
        if i == 0 {
            full_pic = line_pic.to_owned();
        } else {
            full_pic = concatenate!(Axis(0), full_pic, line_pic);
        }
    }

    printfull(&full_pic);
    // now look for monsters
    //                  #
    //#    ##    ##    ###
    // #  #  #  #  #  #

    // transform monster in array:
    let monster: Array2<u32> = Array::from_shape_vec(
        (3, 20),
        vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0,
            0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0,
            0, 0,
        ],
    )
    .unwrap();
    printfull(&monster);

    // move window over full pic and try to find monster by counting tile after bitand
    let mut m_count = 0;
    let monster_size = monster.iter().filter(|&&x| x == 1).count();

    // loop and rotate array until we find the monsters
    let mut i = 0;
    loop {
        for part in full_pic.windows(monster.dim()) {
            let res = &monster & &part.to_owned();
            if res.iter().filter(|&&x| x > 0).count() == monster_size {
                m_count += 1;
            }
        }
        if m_count > 0 {
            break;
        }
        if i < 4 {
            full_pic.swap_axes(0, 1);
            full_pic.invert_axis(Axis(1));
        } else {
            full_pic.invert_axis(Axis(1));
        }
        i = (i + 1) % 5;
    }

    println!("Found {} 🐍 Monsters!!", m_count);
    println!(
        "Part2: # that are not monsters -> {}",
        full_pic.iter().filter(|&&x| x == 1).count() - m_count * monster_size
    );
}

fn printfull(array: &Array2<u32>) {
    for line in array.outer_iter() {
        for it in line.iter() {
            print!("{}", if it == &0 { "." } else { "#" });
        }
        println!();
    }
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = args[1].clone();
    if !file.ends_with(".txt") {
        file += &".txt".to_string();
    }
    println!("Reading = {}", file);
    let text = fs::read_to_string(file).expect("File not found");
    let split = text.trim().split("\n\n");
    let mut tiles: Vec<Tile> = Vec::new();
    for s in split {
        tiles.push(parse_tile(&s.split('\n').collect::<Vec<&str>>()));
    }
    println!("Imported {} tiles", tiles.len());
    puzzle1(&mut tiles);
}
