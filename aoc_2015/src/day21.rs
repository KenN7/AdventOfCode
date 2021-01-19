use crate::input;

pub fn day21() -> input::Result<()> {
    let contents = input::load_day_file("day21.txt");
    let mut content_iter = contents.lines();
    let boss_life: i32 = content_iter
        .next()
        .unwrap()
        .strip_prefix("Hit Points: ")
        .unwrap()
        .parse::<i32>()?;
    let boss_dmg = content_iter
        .next()
        .unwrap()
        .strip_prefix("Damage: ")
        .unwrap()
        .parse::<i32>()?;
    let boss_armor = content_iter
        .next()
        .unwrap()
        .strip_prefix("Armor: ")
        .unwrap()
        .parse::<i32>()?;
    println!(
        "Fighting the boss with {} life, {} damage and {} armor.",
        boss_life, boss_dmg, boss_armor
    );

    let weapons = [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
    let armors = [
        (13, 0, 1),
        (31, 0, 2),
        (53, 0, 3),
        (75, 0, 4),
        (102, 0, 5),
        (0, 0, 0),
    ];
    let rings = [
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
        (0, 0, 0),
    ];

    let mut wins = Vec::new();
    let mut loose = Vec::new();
    for weapon in weapons.iter() {
        for armor in armors.iter() {
            for ring1 in rings.iter() {
                for ring2 in rings.iter() {
                    if ring1 == ring2 && ring1 != &(0, 0, 0) && ring2 != &(0, 0, 0) {
                        continue;
                    }
                    let cost = weapon.0 + armor.0 + ring1.0 + ring2.0;
                    let damage = weapon.1 + ring1.1 + ring2.1;
                    let armor_v = armor.2 + ring1.2 + ring2.2;
                    if play(boss_life, boss_armor, boss_dmg, damage, armor_v) {
                        wins.push(cost);
                    } else {
                        loose.push(cost);
                    }
                }
            }
        }
    }

    println!(
        "Minimal ⚔️ stuff🛡️  to win 🤘 costs: {}",
        wins.iter().min().unwrap()
    );
    println!(
        "Max ⚔️ stuff🛡️  and still loose 👎 costs: {}",
        loose.iter().max().unwrap()
    );

    Ok(())
}

fn play(mut boss_life: i32, boss_armor: i32, boss_dmg: i32, damage: i32, armor: i32) -> bool {
    //play rounds
    let mut life: i32 = 100;
    loop {
        boss_life -= std::cmp::max(damage - boss_armor, 1);
        if boss_life <= 0 {
            // println!(
            //     "After {} rounds, you Won! You: {} - Boss : {}",
            //     round, life, boss_life
            // );
            return true;
        }
        life -= std::cmp::max(boss_dmg - armor, 1);
        if life <= 0 {
            // println!(
            //     "After {} rounds, you lost! You: {} - Boss : {}",
            //     round, life, boss_life
            // );
            return false;
        }
    }
}
