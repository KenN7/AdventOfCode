use crate::input;
use ndarray::{s, Array2};
use onig::Regex;

pub fn day6() -> input::Result<()> {
    let contents = input::load_day_file("day6.txt");
    let re_d = Regex::new(r"([a-z ]+) (\d+),(\d+) through (\d+),(\d+)")?;

    let mut array1 = Array2::<u32>::zeros((1000, 1000));
    let mut array2 = Array2::<u32>::zeros((1000, 1000));

    for l in contents.lines() {
        let cap = re_d.captures(l).ok_or("Regex didnt capture")?;
        let (n1, n2, n3, n4): (usize, usize, usize, usize) = (
            cap.at(2).ok_or("Error cap group")?.parse()?,
            cap.at(3).ok_or("Error cap group")?.parse()?,
            cap.at(4).ok_or("Error cap group")?.parse()?,
            cap.at(5).ok_or("Error cap group")?.parse()?,
        );
        match cap.at(1) {
            Some("turn on") => {
                array1.slice_mut(s![n1..n3 + 1, n2..n4 + 1]).fill(1);
                array2
                    .slice_mut(s![n1..n3 + 1, n2..n4 + 1])
                    .iter_mut()
                    .for_each(|x| *x += 1);
            }
            Some("turn off") => {
                array1.slice_mut(s![n1..n3 + 1, n2..n4 + 1]).fill(0);
                array2
                    .slice_mut(s![n1..n3 + 1, n2..n4 + 1])
                    .iter_mut()
                    .for_each(|x| *x = x.saturating_sub(1));
            }
            Some("toggle") => {
                array1
                    .slice_mut(s![n1..n3 + 1, n2..n4 + 1])
                    .iter_mut()
                    .for_each(|x| *x = (*x + 1) % 2);
                array2
                    .slice_mut(s![n1..n3 + 1, n2..n4 + 1])
                    .iter_mut()
                    .for_each(|x| *x += 2);
            }
            _ => (),
        }
    }

    println!(
        "{}💡 lights are lit.",
        array1.iter().filter(|&&x| x == 1).count()
    );
    println!("total brightness is {} 💡.", array2.iter().sum::<u32>());
    Ok(())
}
