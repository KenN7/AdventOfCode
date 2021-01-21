use crate::input;
use std::collections::VecDeque;

pub fn day22() -> input::Result<()> {
    let contents = input::load_day_file("day22.txt");
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
    println!(
        "Fighting the boss with {} life, {} damage.",
        boss_life, boss_dmg
    );

    let spells = [("MM", 53), ("D", 73), ("S", 113), ("P", 173), ("R", 229)];

    let mana_spent = play(&spells, boss_life, boss_dmg, false);

    println!(
        "Minimum mana spent 🧙 to defeat the boss (easy mode): {}",
        mana_spent.iter().map(|x| x.0).min().unwrap()
    );

    let mana_spent = play(&spells, boss_life, boss_dmg, true);

    println!(
        "Minimum mana spent 🧙 to defeat the boss (hard mode): {}",
        mana_spent.iter().map(|x| x.0).min().unwrap()
    );

    Ok(())
}

fn play(
    spells: &[(&str, i32)],
    boss_life: i32,
    boss_dmg: i32,
    hard: bool,
) -> Vec<(i32, Vec<String>)> {
    let mut mana_spent = Vec::new();
    let mut min_mana = i32::MAX;

    let player = Player::new(50, 500);
    let mut rounds = VecDeque::new();
    for spell in spells.iter() {
        rounds.push_front((player.clone(), boss_life, spell))
    }
    while let Some(mut round) = rounds.pop_back() {
        if round.0.mana_spent > min_mana {
            continue;
        }
        if hard {
            round.0.life -= 1;
            if round.0.life <= 0 {
                continue;
            }
        }
        //player turn
        let effect_dmg = round.0.effects();
        let mut boss_life = round.1 - effect_dmg;
        if boss_life <= 0 {
            // println!("Boss defeated !");
            // println!("{:?} * {:?}", round.0.mana_spent, round.0.spells);
            mana_spent.push((round.0.mana_spent, round.0.spells));
            min_mana = std::cmp::min(round.0.mana_spent, min_mana);
            continue;
        }

        let dmg = match round.0.cast(round.2) {
            Some(d) => d,
            None => continue,
        };
        boss_life -= dmg;
        //boss turn
        let effect_dmg = round.0.effects();
        boss_life -= effect_dmg;
        if boss_life <= 0 {
            // println!("Boss defeated !");
            // println!("{:?} * {:?}", round.0.mana_spent, round.0.spells);
            mana_spent.push((round.0.mana_spent, round.0.spells));
            min_mana = std::cmp::min(round.0.mana_spent, min_mana);
            continue;
        }
        round.0.life -= std::cmp::max(boss_dmg - round.0.armor, 1);
        if round.0.life <= 0 {
            continue;
        }
        for spell in spells.iter() {
            rounds.push_front((round.0.clone(), boss_life, spell))
        }
    }
    mana_spent
}

// fn round(player)

#[derive(Debug, Clone)]
struct Player {
    life: i32,
    mana: i32,
    armor: i32,
    shield: i32,
    poison: i32,
    recharge: i32,
    mana_spent: i32,
    spells: Vec<String>,
}

impl Player {
    fn new(life: i32, mana: i32) -> Self {
        Player {
            life,
            mana,
            armor: 0,
            shield: 0,
            poison: 0,
            recharge: 0,
            mana_spent: 0,
            spells: Vec::new(),
        }
    }

    fn effects(&mut self) -> i32 {
        let mut dmg = 0;
        if self.shield > 0 {
            self.armor = 7;
            self.shield -= 1;
        }
        if self.shield == 0 {
            self.armor = 0;
        }
        if self.recharge > 0 {
            self.mana += 101;
            self.recharge -= 1;
        }
        if self.poison > 0 {
            self.poison -= 1;
            dmg += 3;
        }
        return dmg;
    }

    fn cast(&mut self, spell: &(&str, i32)) -> Option<i32> {
        self.mana -= spell.1;
        if self.mana < 0 {
            return None;
        }
        self.spells.push(spell.0.to_string());
        self.mana_spent += spell.1;
        match spell.0 {
            "MM" => Some(4),
            "D" => {
                self.life += 2;
                Some(2)
            }
            "S" => {
                if self.shield > 0 {
                    return None;
                }
                self.shield = 6;
                Some(0)
            }
            "P" => {
                if self.poison > 0 {
                    return None;
                }
                self.poison = 6;
                Some(0)
            }
            "R" => {
                if self.recharge > 0 {
                    return None;
                }
                self.recharge = 5;
                Some(0)
            }
            _ => None,
        }
    }
}
