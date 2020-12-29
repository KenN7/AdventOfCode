use std::collections::VecDeque;
use std::env;
use std::fs;

fn puzzle(data: &[VecDeque<u32>]) {
    let mut decks = data.to_owned();
    while !decks[0].is_empty() && !decks[1].is_empty() {
        let c1 = decks[0].pop_front().unwrap();
        let c2 = decks[1].pop_front().unwrap();
        if c1 > c2 {
            decks[0].extend(&[c1, c2]);
        } else {
            decks[1].extend(&[c2, c1]);
        }
    }
    println!("{:?}, {:?}", decks[0], decks[1]);
    let winner = &decks[!decks[1].is_empty() as usize];
    let score = winner
        .iter()
        .enumerate()
        .fold(0, |s, (i, &j)| s + (winner.len() as u32 - i as u32) * j);
    println!("Puzzle1: {}", score);
}

fn puzzle2(data: &[VecDeque<u32>]) {
    let mut decks = data.to_owned();

    fn play_game(decks: &mut Vec<VecDeque<u32>>, game: u32) -> u32 {
        // println!("\n=== Game {} ===\n", game);
        // println!("Decks p1: {:?}, p2: {:?}", decks[0], decks[1]);
        let mut history_p1: Vec<VecDeque<u32>> = Vec::new();
        while !decks[0].is_empty() && !decks[1].is_empty() {
            // println!("New round");
            if history_p1.contains(&decks[0]) {
                // println!("Loop, player 1 wins game");
                return 0;
            }
            history_p1.push(decks[0].clone());
            let sub_winner;
            let c1 = decks[0].pop_front().unwrap();
            let c2 = decks[1].pop_front().unwrap();
            // println!("Player 1 plays: {}, Player 2 plays: {}", c1, c2);
            if decks[0].len() as u32 >= c1 && decks[1].len() as u32 >= c2 {
                // println!("Playing subgame..");
                let slice1 = VecDeque::from(Vec::from(decks[0].clone())[..c1 as usize].to_vec());
                let slice2 = VecDeque::from(Vec::from(decks[1].clone())[..c2 as usize].to_vec());
                sub_winner = play_game(&mut vec![slice1, slice2], game + 1);
            // println!("\n<<< Back to Game {} <<<\n", game);
            } else {
                sub_winner = (c2 > c1) as u32;
            }

            if sub_winner == 0 {
                decks[0].extend(&[c1, c2]);
            // println!("Player 1 wins round");
            } else {
                decks[1].extend(&[c2, c1]);
                // println!("Player 2 wins round");
            }
        }
        decks[0].is_empty() as u32
    }

    let winner = play_game(&mut decks, 1) as usize;

    let score = decks[winner].iter().enumerate().fold(0, |s, (i, &j)| {
        s + (decks[winner].len() as u32 - i as u32) * j
    });
    println!("Puzzle 2: {}", score);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<VecDeque<u32>> = content
        .trim()
        .split("\n\n")
        .map(|x| {
            x.split('\n')
                .skip(1)
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    println!("{:?}", data);
    puzzle(&data);
    puzzle2(&data);
}
