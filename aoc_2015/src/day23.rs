use crate::input;
use std::collections::HashMap;

pub fn day23() -> input::Result<()> {
    let contents = input::load_day_file("day23.txt");
    let instr_list = contents
        .lines()
        .map(|x| x.split(' ').map(|x| x.replace(",", "")).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // println!("{:?}", instr_list);

    let regs = exec(0, 0, &instr_list);
    println!("After exec 🖥️  registers are: {:?}", regs);

    let regs = exec(1, 0, &instr_list);
    println!("After exec 🖥️  (with a=1) registers are: {:?}", regs);

    Ok(())
}

fn exec(a: u32, b: u32, instr_list: &Vec<Vec<String>>) -> HashMap<&str, u32> {
    let mut regs = HashMap::new();
    regs.insert("a", a);
    regs.insert("b", b);

    let mut pc: usize = 0;
    while let Some(instr) = instr_list.get(pc) {
        match instr.get(0).as_deref().map(String::as_str) {
            Some("hlf") => {
                let reg = regs.get_mut(instr.get(1).unwrap().as_str()).unwrap();
                *reg = *reg >> 1;
            }
            Some("tpl") => {
                let reg = regs.get_mut(instr.get(1).unwrap().as_str()).unwrap();
                *reg *= 3;
            }
            Some("inc") => {
                let reg = regs.get_mut(instr.get(1).unwrap().as_str()).unwrap();
                *reg += 1;
            }
            Some("jmp") => {
                pc = (pc as i32 + instr.get(1).unwrap().parse::<i32>().unwrap() - 1) as usize
            }
            Some("jie") => {
                if (*regs.get(instr.get(1).unwrap().as_str()).unwrap() as i32).rem_euclid(2) == 0 {
                    pc = (pc as i32 + instr.get(2).unwrap().parse::<i32>().unwrap() - 1) as usize
                }
            }
            Some("jio") => {
                if *regs.get(instr.get(1).unwrap().as_str()).unwrap() == 1 {
                    pc = (pc as i32 + instr.get(2).unwrap().parse::<i32>().unwrap() - 1) as usize
                }
            }
            _ => (),
        }
        // println!("instr: {:?}", instr);
        // println!("pc: {:?} - regs {:?}", pc, regs);
        pc += 1;
    }
    regs
}
