use crate::input;
use rand::{self, Rng};
use std::collections::{HashMap, HashSet};

pub fn day19() -> input::Result<()> {
    let contents = input::load_day_file("day19.txt");
    let transforms = contents
        .lines()
        .rev()
        .skip(2)
        .map(|x| x.split(" => "))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .collect::<Vec<_>>();
    let molecule = contents.lines().rev().next().unwrap();

    // println!("{:?} - {}", transforms, molecule);

    let mut mol_set = HashSet::new();
    for t in transforms.iter() {
        for m in molecule.match_indices(t.0) {
            let mol = molecule[..m.0].to_string() + t.1 + &molecule[m.0 + t.0.len()..];
            mol_set.insert(mol);
        }
    }

    println!("Calibrated {} 🧪 molecules 🦌.", mol_set.len());

    //     // ----------------- test greedy biggest replacement ------------
    //     let mut rng = rand::thread_rng();
    //     let mut mol = molecule.to_string();
    //     let mut step = 0;
    //     loop {
    //         if mol == "e" {
    //             break;
    //         }
    //         let mut max = ("", "");
    //         for t in transforms.iter() {
    //             if mol.match_indices(t.1).count() > 0 {
    //                 if t.1.len() > max.1.len() {
    //                     max = *t;
    //                 }
    //             }
    //         }

    //         // let m = mol.match_indices(max.1).next().unwrap();
    //         // with rng or without rng, no change in solution.
    //         let n = rng.gen_range(0..mol.match_indices(max.1).count());
    //         let m = mol.match_indices(max.1).nth(n).unwrap();
    //         mol = mol[..m.0].to_string() + max.0 + &mol[m.0 + max.1.len()..];
    //         println!("{} - {} - {}", mol, step, n);
    //         step += 1;
    //     }
    // println!("Reconstructed 🧪 molecule in {} steps 🥼.", step);

    // ----------------- test of A* --------------------------
    // let mut ways = vec!["e".to_string()];
    // let mut scores = HashMap::new();
    // scores.insert("e".to_string(), (0, 0));
    // let mut clo = HashSet::new();

    // loop {
    //     let ind = ways
    //         .iter()
    //         .min_by_key(|x| scores.get(*x).unwrap().1)
    //         .unwrap();
    //     let mol = ways.swap_remove(ways.iter().position(|x| x == ind).unwrap());

    //     if mol == molecule {
    //         println!("final:{} - {:?}", mol, scores.get(&mol).unwrap());
    //         break;
    //     }
    //     println!("{} - {:?}", mol, scores.get(&mol).unwrap());
    //     // println!("opl: {:?}, clo: {:?}", ways, clo);
    //     let possibles = transforms.iter().filter(|x| mol.contains(x.0));
    //     for p in possibles {
    //         for m in mol.match_indices(p.0) {
    //             let mol_new = mol[..m.0].to_string() + p.1 + &mol[m.0 + p.0.len()..];
    //             // println!("sc:{:?} - {:?}-", p, scores,);
    //             // println!("new:{}", mol_new);
    //             if !clo.contains(&mol_new)
    //                 && !(ways.contains(&mol_new)
    //                     && scores.get(&mol_new).unwrap().0 > scores.get(&mol).unwrap().0)
    //             {
    //                 scores.entry(mol_new.to_string()).or_insert((0, 0)).0 =
    //                     scores.get_mut(&mol).unwrap().0 + 1;
    //                 scores.get_mut(&mol_new).unwrap().1 +=
    //                     scores.get(&mol_new).unwrap().0 + di(&mol_new, molecule);
    //                 ways.push(mol_new);
    //             }
    //         }
    //     }
    //     clo.insert(mol);
    // }

    // fn di(s1: &String, s2: &str) -> i64 {
    //     let mut count = 0; //s2.len() as i64;
    //     for (l, k) in s2.chars().zip(s1.chars()) {
    //         if l == k {
    //             count += 1;
    //         }
    //     }

    //     s2.len() as i64 - count
    //     // s2.len() as i64 - count + (s2.len() as i64 - s1.len() as i64).abs()
    //     // (s2.len() as i64 - s1.len() as i64).abs() - count
    // }

    // ------------ test of recursion: ---------------
    let mut cache = HashMap::new();
    fn recur(
        mol: &str,
        goal: &str,
        step: u32,
        trans: &Vec<(&str, &str)>,
        cache: &mut HashMap<String, Vec<u32>>,
    ) -> Vec<u32> {
        // println!("{} - {}", mol, step);
        if mol == goal {
            return vec![step];
        }
        if cache.contains_key(mol) {
            return cache.get(&mol.to_string()).unwrap().clone();
        }
        let mut ways = Vec::new();
        let mut possibles = trans
            .iter()
            .filter(|x| mol.contains(x.1))
            .collect::<Vec<_>>();
        possibles.sort_unstable_by_key(|x| x.1.len());
        'out: for p in possibles.iter().rev() {
            for m in mol.rmatch_indices(p.1) {
                let mol_new = mol[..m.0].to_string() + p.0 + &mol[m.0 + p.1.len()..];
                ways.extend(recur(&mol_new, goal, step + 1, trans, cache));
                if !ways.is_empty() {
                    break 'out;
                }
            }
        }
        // println!("{} - {} ", mol, step);
        cache.insert(mol.to_string(), ways.clone());
        ways
    }
    let steps = recur(molecule, "e", 0, &transforms, &mut cache);

    println!(
        "We need {} steps to make the molecule.",
        steps.iter().min().unwrap()
    );

    Ok(())
}
