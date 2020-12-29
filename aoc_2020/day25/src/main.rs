// use std::collections::HashMap;

fn puzzle(door_pkey: u64, card_pkey: u64) {
    let mut sub = 7;
    let mut ret = 1;
    let mut loop_s_door = 0;
    while ret != door_pkey {
        ret = (sub * ret).rem_euclid(20201227);
        loop_s_door += 1;
    }
    let mut loop_s_card = 0;
    ret = 1;
    while ret != card_pkey {
        ret = (sub * ret).rem_euclid(20201227);
        loop_s_card += 1;
    }
    println!("loop size for card: {}, door: {}", loop_s_card, loop_s_door);

    sub = card_pkey;
    ret = 1;
    for _s in 0..loop_s_door {
        ret = (sub * ret).rem_euclid(20201227);
    }
    println!("encrypt key: {}", ret);
}

fn main() {
    let door_pkey = 9033205;
    let card_pkey = 9281649;

    // let card_pkey = 5764801;
    // let door_pkey = 17807724;

    puzzle(door_pkey, card_pkey);
}
