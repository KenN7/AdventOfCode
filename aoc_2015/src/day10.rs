use crate::input;

pub fn day10() -> input::Result<()> {
    let contents = input::load_day_file("day10.txt");

    let mut seq: Vec<_> = Vec::new();
    let mut current = contents.trim().to_string();
    for _r in 0..50 {
        let mut it = current.chars();
        let mut prev = '\0';
        let mut count = 1;
        let mut output = String::new();
        while let Some(c) = it.next() {
            if c == prev {
                count += 1;
            } else if c != prev && prev != '\0' {
                output.push_str(&format!("{}{}", count, prev));
                count = 1;
            }
            prev = c;
        }
        output.push_str(&format!("{}{}", count, prev));
        seq.push(output.clone());
        current = output;
    }

    println!(
        "🔢 Len of sequence after 40 rounds: {:?}, 📒 and after 50 rounds: {}",
        seq[39].len(),
        seq[49].len()
    );

    Ok(())
}
