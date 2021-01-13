use crate::input;
use std::collections::HashMap;

pub fn day7() -> input::Result<()> {
    let contents = input::load_day_file("day7.txt");

    let mut circuit: HashMap<_, _> = contents
        .lines()
        .map(|x| x.split(" -> ").collect::<Vec<_>>())
        .map(|i| (i[1], i[0]))
        .collect();

    // println!("{:?}", circuit);

    fn recur(circuit: &HashMap<&str, &str>, solve: &str, cache: &mut HashMap<String, u16>) -> u16 {
        let eq = circuit.get(solve).expect("Cannot get key {} in hashmap");
        let ret;
        if let Some(r) = cache.get(eq.to_owned()) {
            return *r;
        }
        // println!("{}", eq);
        let p = eq.parse::<u16>();
        match p {
            Ok(i) => {
                ret = i;
            }
            Err(_e) => {
                if let Some(op) = eq.strip_prefix("NOT ") {
                    ret = !op
                        .parse::<u16>()
                        .unwrap_or_else(|_| recur(circuit, op, cache));
                } else if eq.contains("OR") || eq.contains("AND") {
                    let op: Vec<_> = eq.split(' ').collect();
                    let op1 = op[0]
                        .parse::<u16>()
                        .unwrap_or_else(|_| recur(circuit, op[0], cache));
                    let op2 = op[2]
                        .parse::<u16>()
                        .unwrap_or_else(|_| recur(circuit, op[2], cache));
                    if eq.contains("OR") {
                        ret = op1 | op2;
                    } else {
                        ret = op1 & op2;
                    }
                } else if eq.contains("LSHIFT") {
                    let op: Vec<_> = eq.split(" LSHIFT ").collect();
                    ret = (recur(circuit, op[0], cache)) << op[1].parse::<u16>().unwrap();
                } else if eq.contains("RSHIFT") {
                    let op: Vec<_> = eq.split(" RSHIFT ").collect();
                    ret = (recur(circuit, op[0], cache)) >> op[1].parse::<u16>().unwrap();
                } else {
                    ret = recur(circuit, eq, cache);
                }
            }
        }
        cache.insert(eq.to_string(), ret);
        ret
    }

    let mut cache = HashMap::new();
    let val = recur(&circuit, "a", &mut cache);
    println!("Value of wire 🔌: {}", val);

    cache.clear();
    let val_str = format!("{}", val);
    circuit.insert("b", val_str.as_str());
    let val = recur(&circuit, "a", &mut cache);
    println!("Overriden wire value 🔌⚡: {}", val);

    Ok(())
}
