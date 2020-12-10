use std::env;
use std::fs;
use std::collections::BTreeMap;


fn puzzle(data: &mut Vec<u32>) {
    data.push(0);
    data.sort();
    let data2 = data.windows(2).map(|x| x[1] - x[0]).collect::<Vec<u32>>();
    let count1 = data2.iter().filter(|x| **x == 1);
    let count3 = data2.iter().filter(|x| **x == 3);

    println!("{:?}", data);
    println!("{:?}", data2);

    let mut vec3: Vec<u64> = vec![0; data.len()];
    vec3[0] = 1;
    for (i, item) in data.iter().enumerate() {
        for j in (0..i).rev() {
            if *item > data[j] + 3 {
                break;
            }
            vec3[i] += vec3[j];
        }
    }

    let mut cache = BTreeMap::new();

    fn recur(index: usize, data: &Vec<u32>, cache: &mut BTreeMap<usize,u64>) -> u64 {
        if index <= 0 {
            return 1;
        }
        let mut count: u64 = 0;
        for i in 1..4 {
            if index >= i && data[index] - data[index - i] <= 3 {
                if cache.contains_key(&(index-i)) {
                    count += *cache.get(&(index - i)).unwrap()
                } else {
                    count += recur(index - i, data, cache);
                }
            }
        }
        // println!("count {}", count);
        cache.insert(index, count);
        count
    }
    let count = recur(data.len() - 1, &data, &mut cache);

    println!("puzzle1: {:?}", count1.count() * (count3.count() + 1));
    
    println!("recursion puzzle2: {}", count);
    println!("dynamic prog puzzle2 : {}", vec3[vec3.len()-1]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);

    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut data: Vec<u32> = content
        .trim()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    puzzle(&mut data);
}
