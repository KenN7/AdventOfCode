use crate::input;
use onig::Regex;

pub fn day5() {
    let contents = input::load_day_file("day5.txt");
    let mut count = 0;
    let re_d = Regex::new(r"([a-z])\1").unwrap();
    for line in contents.lines() {
        let nice_v = line
            .chars()
            .filter(|x| ['a', 'e', 'i', 'o', 'u'].contains(x))
            .count()
            >= 3;
        // let nice_d = line.chars().tuple_windows().filter(|(x, y)| x == y).count() != 0;
        let nice_d = re_d.find_iter(line).count() != 0;
        if nice_v
            && nice_d
            && !line.contains("ab")
            && !line.contains("cd")
            && !line.contains("pq")
            && !line.contains("xy")
        {
            count += 1;
        }
    }
    println!("Nb of nice 🤗 strings: {}", count);

    let re_aba = Regex::new(r"([a-z]).\1").unwrap();
    let re_dd = Regex::new(r"([a-z][a-z]).*\1").unwrap();
    let part2 = contents
        .lines()
        .filter(|x| re_dd.find(x).is_some() && re_aba.find(x).is_some());

    println!("Nb of very nice 🤗 strings: {}", part2.count());
}
