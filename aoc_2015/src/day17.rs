use crate::input;

pub fn day17() -> input::Result<()> {
    let contents = input::load_day_file("day17.txt");
    let jars = contents
        .lines()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    fn recur(
        jars: &Vec<u32>,
        used: Vec<u32>,
        skip: usize,
        total: u32,
        combinations: &mut Vec<Vec<u32>>,
    ) -> u32 {
        if used.iter().sum::<u32>() == total {
            combinations.push(used);
            return 1;
        } else if used.iter().sum::<u32>() > total {
            return 0;
        }
        let mut total = 0;
        for (i, jar) in jars.iter().enumerate().skip(skip) {
            let mut used2 = used.clone();
            used2.push(*jar);
            total += recur(jars, used2, i + 1, 150, combinations);
        }
        total
    }

    let mut combinations = Vec::new();
    let total = recur(&jars, Vec::new(), 0, 150, &mut combinations);
    println!("There are {} container 🪣 combinations for 150L.", total);

    let min_combi = combinations.iter().min_by_key(|x| x.len()).unwrap().len();
    println!(
        "The minimal 🪣 combination size is {} and there are {} such combinations.",
        min_combi,
        combinations.iter().filter(|x| x.len() == min_combi).count()
    );

    Ok(())
}
