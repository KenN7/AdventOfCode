use crate::input;
// use itertools::Itertools;

pub fn day2() -> input::Result<()> {
    let contents = input::load_day_file("day2.txt");

    // This impl is a little bit faster but does not handle well errors
    // let data: Vec<Vec<i32>> = contents
    //     .lines()
    //     .map(|x| x.split('x').map(|x| x.parse().unwrap()).sorted().collect())
    //     .collect();

    let mut data = contents
        .lines()
        .map(|x| x.split('x').map(|x| x.parse()).collect())
        .collect::<Result<Vec<Vec<i32>>, _>>()?;
    data.iter_mut().for_each(|x| x.sort_unstable());

    // Vec l x w x h, sorted
    // 2*l*w + 2*w*h + 2*h*l + smallest area

    let total_ft: Vec<i32> = data
        .iter()
        .map(|x| 2 * x[0] * x[1] + 2 * x[1] * x[2] + 2 * x[2] * x[0] + x[0] * x[1])
        .collect();
    println!(
        "Total wrapping paper size needed 🎁: {} ft",
        total_ft.iter().sum::<i32>()
    );

    let total_rb: Vec<i32> = data
        .iter()
        .map(|x| x[0] * 2 + x[1] * 2 + x[0] * x[1] * x[2])
        .collect();
    println!(
        "Total ribbon needed 🎀: {} ft",
        total_rb.iter().sum::<i32>()
    );
    Ok(())
}
