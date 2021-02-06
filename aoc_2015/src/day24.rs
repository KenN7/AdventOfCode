use crate::input;
use itertools::Itertools;

pub fn day24() -> input::Result<()> {
    let contents = input::load_day_file("day24.txt");
    let packages = contents
        .lines()
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let weight: u64 = packages.iter().sum::<u64>();

    // println!("{:?} -> {} -> {}", packages, nb_packages, weight_per_bag);

    fn recur(packages: Vec<u64>, weight_per_bag: u64, mut prod: u64, depth: u32, max: u32) -> u64 {
        let sets = packages.iter().rev().powerset();
        for s in sets {
            if s.iter().copied().sum::<u64>() == weight_per_bag {
                let mut diff = packages.clone();
                for e in s.iter() {
                    diff.swap_remove(diff.iter().position(|x| &x == e).unwrap());
                }
                if depth == 0 {
                    prod = s.iter().copied().product::<u64>()
                } else if depth == max && diff.is_empty() {
                    return prod;
                }
                prod = recur(diff, weight_per_bag, prod, depth + 1, max);
                if prod != 0 {
                    break;
                }
            }
        }
        prod
    }

    println!(
        "Minimal QE for 3 bags of 🎁 gifts 💰 {}",
        recur(packages.clone(), weight / 3, 0, 0, 2)
    );
    println!(
        "Minimal QE for 4 bags of 🎁 gifts 💰 {}",
        recur(packages.clone(), weight / 4, 0, 0, 3)
    );

    Ok(())
}
