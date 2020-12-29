use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

fn puzzle(rules: HashMap<&str, Vec<Vec<String>>>, messages: Vec<&str>) {
    //fn recur(
    //    to_dec: &String,
    //    rules: &HashMap<&str, Vec<Vec<String>>>,
    //    cache: &mut HashMap<String, Vec<String>>,
    //) -> Vec<String> {
    //    let sub_r = rules.get(to_dec.as_str()).unwrap();
    //    let mut list: Vec<String> = Vec::new();
    //    // println!("to_dec: {}",to_dec);
    //    // println!("sub: {:?}", sub_r);
    //    if sub_r.len() > 0 {
    //        // println!(" or list");
    //        //its an OR
    //        for sub in sub_r.iter() {
    //            let mut branch: Vec<Vec<String>> = Vec::new();
    //            for elts in sub.iter() {
    //                if elts.chars().all(|x| x.is_alphabetic()) {
    //                    branch.push(vec![elts.to_string()]);
    //                } else {
    //                    if cache.contains_key(elts) {
    //                        branch.push(cache.get(elts).unwrap().clone());
    //                    } else {
    //                        branch.push(recur(elts, &rules, cache));
    //                    }
    //                }
    //            }
    //            if branch.len() > 1 {
    //                let mut tmp_branch = branch[0].clone();
    //                let mut tmp_values = Vec::new();
    //                for br in branch.iter().skip(1) {
    //                    tmp_values.clear();
    //                    for elt2 in br.iter() {
    //                        for elt in tmp_branch.iter() {
    //                            // list.push(format!("{}{}", elt, elt2));
    //                            tmp_values.push(format!("{}{}", elt, elt2));
    //                        }
    //                    }
    //                    tmp_branch = tmp_values.clone();
    //                }
    //                list.extend(tmp_branch);
    //            } else {
    //                list.extend(branch[0].clone());
    //            }

    //            // println!("b1: {:?}", branch);
    //        }
    //    // println!("b2: {:?}", branch2);
    //    } else {
    //        //should not happen
    //        println!("Panic! len more than 1");
    //    }

    //    // println!("to_dec: {}", to_dec);
    //    // println!("list: {:?}", list);

    //    cache.insert(to_dec.to_string(), list.clone());
    //    list
    //}
    // let mut cache = HashMap::new();
    // let list = recur(&"0".to_string(), &rules, &mut cache);
    // println!("final list:{:?}", list);
    // println!("final list:{:?}", list.len());

    // let set: HashSet<&str> = list.iter().map(|x| x.as_ref()).collect();
    // let count = messages.iter().filter(|&x| set.contains(x)).count();
    // println!("{}", count);

    fn test(mess: &str, rules: &HashMap<&str, Vec<Vec<String>>>, start: &String) -> HashSet<u32> {
        // println!("m:{} s:{}", mess, start);
        let sub_r = rules.get(start.as_str()).unwrap();
        let mut set: HashSet<u32> = HashSet::new();
        if sub_r[0][0].chars().all(|x| x.is_alphabetic()) {
            if mess.len() > 0 && mess[0..1] == sub_r[0][0][0..1] {
                set.insert(1);
            }
        } else {
            for lr in sub_r.iter() {
                let mut r_set: HashSet<u32> = HashSet::new();
                r_set.insert(0);
                for rule in lr.iter() {
                    let mut new_set: HashSet<u32> = HashSet::new();
                    for n in r_set.iter() {
                        let mut sub_m_set: HashSet<u32> = HashSet::new();
                        let m_set: HashSet<u32> = test(&mess[*n as usize..], rules, rule);
                        for m in m_set.iter() {
                            sub_m_set.insert(m + n);
                        }
                        
                        new_set = new_set.union(&sub_m_set).map(|x| *x).collect();
                    }
                    r_set = new_set;
                    // println!("{:?}",r_set);
                }
                set = set.union(&r_set).map(|x| *x).collect();
            }
        }
        set
    }


    let set_list: Vec<bool> = messages.iter().map(|x| test(x,&rules,&"0".to_string()).contains(&(x.len() as u32))).collect();
    
    println!("{:?}", set_list);
    println!("{:?}", set_list.iter().filter(|&&x| x == true).count());

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("reading file {}", filename);
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let data: Vec<&str> = content.trim().split("\n\n").collect();

    let rules: HashMap<_, Vec<Vec<String>>> = data[0]
        .lines()
        .map(|x| x.split(":").collect::<Vec<&str>>())
        .map(|x| {
            (
                x[0],
                x[1].trim()
                    .replace("\"", "")
                    .split(" | ")
                    .map(|x| x.split(" ").map(|x| x.to_string()).collect())
                    .collect(),
            )
        })
        .collect();
    println!("{:?}", rules);
    let list: Vec<_> = data[1].lines().collect();
    println!("{:?}", list);

    puzzle(rules, list);
}
