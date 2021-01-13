use crate::input;
use onig::Regex;
use std::collections::HashMap;

fn filter_by<'a, T>(
    map: &HashMap<&'a str, HashMap<&'a str, i32>>,
    filters: &[(&str, i32)],
    comp: T,
) -> HashMap<&'a str, HashMap<&'a str, i32>>
where
    T: Fn(i32, i32) -> bool,
{
    let mut filtered_map = map.clone();
    for filter in filters.iter() {
        // println!("{:?}", filter);
        filtered_map = filtered_map
            .iter()
            .filter(|&x| x.1.get(filter.0).is_none() || comp(*x.1.get(filter.0).unwrap(), filter.1))
            // .filter(|x| x.1.get(filter.0).unwrap() == &filter.1)
            .map(|x| (*x.0, x.1.clone()))
            .collect();
    }
    filtered_map
}

pub fn day16() -> input::Result<()> {
    let contents = input::load_day_file("day16.txt");
    let re_sue = Regex::new(r"Sue (\d+): ").unwrap();
    let re_features = Regex::new(r"([a-z]+): (\d+)").unwrap();

    let mut map = HashMap::new();
    for line in contents.lines() {
        let sue = re_sue.captures(line).unwrap().at(1).unwrap();
        // println!("{}", sue);
        let mut temp = HashMap::new();
        for cap in re_features.captures_iter(line) {
            let feature = cap.at(1).unwrap();
            let number = cap.at(2).unwrap().parse::<i32>().unwrap();
            temp.insert(feature, number);
        }
        map.insert(sue, temp);
    }
    // println!("{:?}", map);

    //to filter:
    let to_filter = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];

    let filtered_map = filter_by(&map, &to_filter, |a, b| -> bool { a == b });
    println!(
        "First guess of 👵 Aunt Sue is: {:?}",
        filtered_map.iter().next().unwrap().0
    );

    let to_filter = [
        ("children", 3),
        ("samoyeds", 2),
        ("akitas", 0),
        ("vizslas", 0),
        ("cars", 2),
        ("perfumes", 1),
    ];

    let mut filtered_map = filter_by(&map, &to_filter, |a, b| -> bool { a == b });

    let filter_gt = [("cats", 7), ("trees", 3)];
    filtered_map = filter_by(&filtered_map, &filter_gt, |a, b| -> bool { a > b });

    let filter_lt = [("pomeranians", 3), ("goldfish", 5)];
    filtered_map = filter_by(&filtered_map, &filter_lt, |a, b| -> bool { a < b });

    println!(
        "Second guess of 👵 Aunt Sue is: {:?}",
        filtered_map.iter().next().unwrap().0
    );

    Ok(())
}
