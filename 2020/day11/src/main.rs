use ndarray::prelude::*;
use std::cmp;
use std::env;
use std::fs;

fn pprint(data: &Array2<char>) {
    for row in data.genrows() {
        println!();
        for seat in row.iter() {
            print!("{}", seat);
        }
    }
    println!();
}

fn adjacents(data: &Array2<char>, i: usize, j: usize) -> usize {
    let x = (i.saturating_sub(1), cmp::min(i + 2, data.dim().0));
    let y = (j.saturating_sub(1), cmp::min(j + 2, data.dim().1));
    let slice = data.slice(s![x.0..x.1, y.0..y.1]);
    // println!("{:?}", slice);

    slice
        .iter()
        .filter(|x| **x == '#')
        .count()
        .saturating_sub((data[[i, j]] == '#') as usize)
}

fn get_seat(data: &Array2<char>, mut i: i32, mut j: i32, dir: &(i32, i32)) -> char {
    j += dir.1;
    i += dir.0;
    while let Some(c) = data.get((i as usize, j as usize)) {
        if c == &'#' {
            return '#';
        } else if c == &'L' {
            return 'L';
        }
        j += dir.1;
        i += dir.0;
    }
    '.'
}

fn sight(data: &Array2<char>, i: usize, j: usize) -> usize {
    let seats = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ]
    .iter()
    .map(|x| get_seat(data, i as i32, j as i32, x));

    seats.filter(|x| *x == '#').count()
}

fn puzzle(mut data: Array2<char>, limit: usize, adj_f: &dyn Fn(&Array2<char>, usize, usize) -> usize) {
    let mut copy = data.clone();
    'out: loop {
        for ((i, j), seat) in data.indexed_iter() {
            let adj;
            adj = adj_f(&data, i, j);
            // println!("{},{}->{}", i, j, seat);
            // println!("{:?}", adj);
            if seat == &'L' && adj == 0 {
                copy[[i, j]] = '#';
            } else if seat == &'#' && adj >= limit {
                copy[[i, j]] = 'L';
            }
        }
        if data == copy {
            break 'out;
        }
        data.assign(&copy)
    }

    // pprint(&copy);
    println!("Puzzle: {}", data.iter().filter(|x| **x == '#').count());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let height = content.lines().count();
    let width = (content.lines().next().unwrap()).chars().count();

    let data = content.trim().replace('\n', "").chars().collect();

    let array = Array::from_shape_vec((height, width), data).unwrap();

    println!("{:?}", array);
    puzzle(array.clone(), 4, &adjacents);
    puzzle(array, 5, &sight);
}
