use crate::input;
use ndarray::{s, Array2};
use std::cmp::min;

fn get_on_neighbors(x: usize, y: usize, array: &Array2<u32>) -> u32 {
    let area = array.slice(s![
        x.saturating_sub(1)..=min(array.dim().0 - 1, x + 1),
        y.saturating_sub(1)..=min(array.dim().1 - 1, y + 1)
    ]);

    // println!("{:#?}", area);
    (area.iter().filter(|&&x| x == 1).count() as u32).saturating_sub(array[[x, y]] as u32)
}

pub fn day18() -> input::Result<()> {
    let contents = input::load_day_file("day18.txt");
    let vec = contents
        .lines()
        .map(|x| x.chars().map(|x| (x == '#') as u32))
        .flatten()
        .collect::<Vec<_>>();
    let mut array = Array2::from_shape_vec((100, 100), vec)?;

    // println!("{:#?}", array);

    let part2 = array.clone();
    let mut clone = array.clone();
    for _r in 0..100 {
        for ((x, y), &light) in array.indexed_iter() {
            let on_neigh = get_on_neighbors(x, y, &array);
            if light == 1 && on_neigh != 2 && on_neigh != 3 {
                clone[[x, y]] = 0;
            } else if light == 0 && on_neigh == 3 {
                clone[[x, y]] = 1;
            }
        }
        array.assign(&clone);
    }

    let on = array.iter().sum::<u32>();
    println!("{} 💡 lights are on.", on);

    array.assign(&part2);
    array[[0, 0]] = 1;
    array[[99, 99]] = 1;
    array[[0, 99]] = 1;
    array[[99, 0]] = 1;
    let mut clone: Array2<u32> = array.clone();
    for _r in 0..100 {
        for ((x, y), &light) in array.indexed_iter() {
            let on_neigh = get_on_neighbors(x, y, &array);
            if light == 1 && on_neigh != 2 && on_neigh != 3 {
                clone[[x, y]] = 0;
            } else if light == 0 && on_neigh == 3 {
                clone[[x, y]] = 1;
            }
            if (x, y) == (99, 99) || (x, y) == (0, 0) || (x, y) == (99, 0) || (x, y) == (0, 99) {
                clone[[x, y]] = 1;
            }
        }
        array.assign(&clone);
    }

    let on = array.iter().sum::<u32>();
    println!("{} 💡 lights are on when corners are always on.", on);

    Ok(())
}
