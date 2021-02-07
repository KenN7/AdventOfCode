use crate::input;

pub fn day25() -> input::Result<()> {
    let row = 2978;
    let col = 3083;

    // for this row and col we want the nth number, where n:
    let n = cantor_coupling(row, col);

    let mut num = 20151125;
    for _i in 1..n {
        num = next_code(num);
    }
    println!("The code for the machine 🖥️  is {}", num);

    Ok(())
}

fn next_code(code: u64) -> u64 {
    (code * 252533).rem_euclid(33554393)
}

fn cantor_coupling(x: u64, y: u64) -> u64 {
    // x->line, y->col
    (x + y - 1) * (x + y - 2) / 2 + y
}
