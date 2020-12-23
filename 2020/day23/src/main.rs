use std::collections::HashSet;

fn puzzle(data: &Vec<usize>, turns: u32) {
    let mut links: Vec<usize> = vec![0; data.len() + 1];
    for x in data.windows(2) {
        links[x[0]] = x[1];
    }
    links[data[data.len() - 1]] = data[0];
    let mut current_cup = data[0];

    for _t in 1..turns + 1 {
        // println!("--- move {} ---", t);
        let c1 = links[current_cup];
        let c2 = links[c1];
        let c3 = links[c2];
        links[current_cup] = links[c3];
        let picks: HashSet<usize> = vec![c1, c2, c3, 0].iter().copied().collect();
        let mut dest = current_cup - 1;
        while picks.contains(&dest) {
            dest = (dest as i32 - 1).rem_euclid(data.len() as i32 + 1) as usize;
        }
        links[c3] = links[dest];
        links[dest] = c1;
        // println!("dest: {}", dest);
        // println!("next: {}", links[current_cup]);
        current_cup = links[current_cup];
    }

    if links.len() < 100 {
        //as long as its between 11 and 1M, print puzzle1
        print!("Puzzle 1: ");
        extract_list(&links);
    } else {
        print!("Puzzle 2, prod :");
        println!("{}", links[1] * links[links[1]]);
    }
}

fn extract_list(links: &Vec<usize>) {
    let mut index = 1;
    // print!("{} ", index);
    for _i in 0..links.len() - 2 {
        print!("{}", links[index]);
        index = links[index];
    }
    println!();
}

fn main() {
    let _example = "389125467";
    let input = "739862541";

    let mut data: Vec<usize> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    puzzle(&data, 100);

    for i in 10..1_000_001 {
        data.push(i);
    }
    puzzle(&data, 10_000_000);
}
