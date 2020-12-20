use ndarray::prelude::*;
use std::collections::{HashMap,HashSet};
use std::env;
use std::fs;

#[derive(Debug)]
struct Tile {
    id: i32,
    edges: Vec<Array1<u32>>,
    data: Array2<u32>,
    position: [i32; 2],
    rotation: i32,
}

impl Tile {
    fn new() -> Tile {
        Tile {
            id: 0,
            edges: vec![Array1::<u32>::zeros(10); 8],
            data: Array2::<u32>::zeros((10, 10)),
            position: [0, 0],
            rotation: 0,
        }
    }

    fn rotate90(&mut self) {
        self.data.swap_axes(0, 1);
        self.data.invert_axis(Axis(1));
    }

    fn flip1(&mut self) {
        self.data.invert_axis(Axis(1));
    }

    fn flip0(&mut self) {
        self.data.invert_axis(Axis(0));
    }

    fn fliprot_until(&mut self)
    {

    }

    fn get_edges(&self) -> Vec<Array1<u32>> {
        let mut edges: Vec<Array1<u32>> = Vec::new();
        edges.push(self.data.row(0).to_owned());
        edges.push(self.data.row(9).to_owned());
        edges.push(self.data.column(0).to_owned());
        edges.push(self.data.column(9).to_owned());
        edges
    }

    fn compute_edges(&mut self) {
        self.edges[0].assign(&self.data.row(0));
        self.edges[2].assign(&self.data.row(9));
        self.edges[1].assign(&self.data.column(0));
        self.edges[3].assign(&self.data.column(9));

        self.edges[4].assign(
            &self
                .data
                .row(0)
                .iter()
                .rev()
                .map(|&x| x)
                .collect::<Array1<u32>>(),
        );
        self.edges[6].assign(
            &self
                .data
                .row(9)
                .iter()
                .rev()
                .map(|&x| x)
                .collect::<Array1<u32>>(),
        );
        self.edges[5].assign(
            &self
                .data
                .column(0)
                .iter()
                .rev()
                .map(|&x| x)
                .collect::<Array1<u32>>(),
        );
        self.edges[7].assign(
            &self
                .data
                .column(9)
                .iter()
                .rev()
                .map(|&x| x)
                .collect::<Array1<u32>>(),
        );
        //println!("ID {}, edges {:?}", self.id, self.edges);
    }
}

fn parse_tile(input: &Vec<&str>) -> Tile {
    let mut tile = Tile::new();
    tile.id = input[0]
        .split(" ")
        .nth(1)
        .unwrap()
        .replace(":", "")
        .parse()
        .unwrap();
    let mut ix: usize = 0;
    for line in input.iter().skip(1) {
        //println!("Tile {}, line = {}", id, line);
        for (col, data) in line.chars().enumerate() {
            tile.data[[ix, col]] = (data == '#') as u32;
        }
        ix += 1;
    }
    tile.compute_edges();
    //println!("tile = {:?}", tile);
    tile
}

fn puzzle1(tiles: &mut Vec<Tile>) {
    let mut all_edges: HashMap<Array1<u32>, (i32, Vec<i32>)> = HashMap::new();
    for t in tiles.iter() {
        for ed in 0..8 {
            let entry = all_edges
                .entry(t.edges[ed].clone())
                .or_insert((0, Vec::new()));
            entry.0 += 1;
            entry.1.push(t.id);
        }
    }
    // println!("Hashmap : {:?}", all_edges);

    let edges_tiles = all_edges
        .iter()
        .filter(|(_edge, data)| data.0 == 1)
        .map(|(_edge, data)| data.1[0])
        .collect::<Vec<i32>>();
    let mut corners: HashMap<i32, i32> = HashMap::new();
    edges_tiles.iter().for_each(|&x| {
        *corners.entry(x).or_insert(0) += 1;
    });
    let res: i64 = corners.iter().filter(|(_id, val)| **val == 4).map(|(&i,_x)| i as i64).product(); 

    // println!("edges tiles : {:?}", edges_tiles);
    println!("corners : {:?}", corners);
    println!("Result = {}", res);

    //select one corner and get good edges
    let first_corner: i32 = corners.iter().filter(|(_id, val)| **val == 4).map(|(&i,_x)| i).nth(0).unwrap();
     let edges_tiles_vals: HashMap<Array1<u32>, (i32, Vec<i32>)>= all_edges
        .iter()
        .filter(|(_edge, data)| data.1[0] == first_corner && data.0 == 1)
        .map(|(x,y)| (x.clone(),y.clone()))
        .collect();
    println!("edges tiles : {:?}", edges_tiles_vals);
    println!("first corner : {}", first_corner);
    
    let mut tiles_map:HashMap<i32,&mut Tile> = tiles.iter_mut().map(|x| (x.id, x) ).collect();

    let first_tile = tiles_map.get_mut(&first_corner).unwrap();

    let mut good_edges: Vec<Array1<u32>> = Vec::new();
    for edge in edges_tiles_vals.keys() {
        if first_tile.get_edges().iter().any(|x| x == edge){
            println!("good edge is {:?}", edge);
            good_edges.push(edge.clone())
        }
    }
    // flip and rotate until edges are on top left
    'out: for flip in 0..3 {
        if flip == 1 { first_tile.flip0(); }
        else if flip == 2 {  first_tile.flip1(); }
        for _rot in 0..4 {
            println!("col: {} as good0: {}",first_tile.data.row(0), good_edges[0] == first_tile.data.row(0));
            println!("ro: {} as good1 {}",first_tile.data.column(0), good_edges[1] == first_tile.data.column(0));
            if good_edges[0] == first_tile.data.row(0) && good_edges[1] == first_tile.data.column(0) 
                || good_edges[0].slice(s![..;-1]) == first_tile.data.row(0) && good_edges[1] == first_tile.data.column(0)
                || good_edges[0] == first_tile.data.row(0) && good_edges[1].slice(s![..;-1]) == first_tile.data.column(0) {
                println!("Rotated till sucess! ", );
                break 'out;
            }
            // println!("{:?}", first_tile.data);
            first_tile.rotate90();
        }
    }



    //compute neighbors of each tile
    let mut neigh: HashMap<i32,HashSet<i32>> = HashMap::new();
    for (_arr, (nb, ids )) in all_edges.iter(){
        if *nb > 1 {
            neigh.entry(ids[0]).or_insert(HashSet::new()).insert(ids[1]);
            neigh.entry(ids[1]).or_insert(HashSet::new()).insert(ids[0]);
        }
    }
    // println!("neighbors : {:?}", neigh);

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
    println!("{} tiles importes", tiles.len());
    puzzle1(&mut tiles);
}
