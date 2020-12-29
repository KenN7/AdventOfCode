use std::env;
use std::fs;

fn puzzle1(data: &[(&str, f32)]) {
    let mut pos = (0f32, 0f32);
    let mut angle = 270f32;

    for (instr, nb) in data.iter() {
        // println!("pos:{:?}, angle:{}", pos, angle);
        match *instr {
            "N" => pos.1 += nb,
            "S" => pos.1 -= nb,
            "E" => pos.0 += nb,
            "W" => pos.0 -= nb,
            "L" => angle = (angle + nb) % 360f32,
            "R" => angle = (angle - nb) % 360f32,
            "F" => match angle as i32 {
                0 | -360 => pos.1 += nb,
                90 | -270 => pos.0 -= nb,
                180 | -180 => pos.1 -= nb,
                270 | -90 => pos.0 += nb,
                _ => println!("Something weird happened angle:{}", angle),
            },
            _ => println!("Something weird happened instr:{}", instr),
        }
    }
    println!("pos:{:?}, angle:{}", pos, angle);
    println!("puz1: {}", pos.1.abs() + pos.0.abs());
}

fn puzzle2(data: &[(&str, f32)]) {
    let mut pos = (0f32, 0f32);
    let mut pos_w = (10f32, 1f32);
    for (instr, nb) in data.iter() {
        let mov = (pos_w.0 - pos.0, pos_w.1 - pos.1);
        match *instr {
            "N" => pos_w.1 += nb,
            "S" => pos_w.1 -= nb,
            "E" => pos_w.0 += nb,
            "W" => pos_w.0 -= nb,
            "L" => {
                pos_w.0 = pos.0 + mov.0 * nb.to_radians().cos() - mov.1 * nb.to_radians().sin();
                pos_w.1 = pos.1 + mov.1 * nb.to_radians().cos() + mov.0 * nb.to_radians().sin();
            }
            "R" => {
                pos_w.0 =
                    pos.0 + mov.0 * (-nb).to_radians().cos() - mov.1 * (-nb).to_radians().sin();
                pos_w.1 =
                    pos.1 + mov.1 * (-nb).to_radians().cos() + mov.0 * (-nb).to_radians().sin();
            }
            "F" => {
                pos_w = (pos_w.0 + nb * mov.0, pos_w.1 + nb * mov.1);
                pos = (pos.0 + nb * mov.0, pos.1 + nb * mov.1);
            }

            _ => println!("Something weird happened instr:{}", instr),
        }
    }
    println!("pos:{:?}, pos_w:{:?}", pos, pos_w);
    println!("puz2: {}", pos.1.abs() + pos.0.abs());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<(&str, f32)> = content
        .trim()
        .split('\n')
        .map(|x| (&x[..1], (&x[1..]).parse().unwrap()))
        .collect();

    // println!("{:?}", data);
    puzzle1(&data);
    puzzle2(&data);
}
