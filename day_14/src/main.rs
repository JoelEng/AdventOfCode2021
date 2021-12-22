use std::collections::HashMap;
use itertools::Itertools;

const INPUT_FILE: &str = "input.txt";

fn main() {
    println!("Answer part one: {}", solve(10));
    println!("Answer part two: {}", solve(40));
    println!("Highest possible answer with u128: {}", solve(126));
}

fn get_input() -> (HashMap<String, u128>, HashMap<char, u128>, HashMap<String, (String, String)>) {
    let input = std::fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file");
    let mut lines = input.lines();
    
    let mut rules = HashMap::new();
    let init_vals = lines.next().unwrap().to_string();
    let mut polymer = HashMap::new();
    let mut counter = HashMap::new();

    lines.next(); //Remove blank line

    //Create rules
    for line in lines {
        let pair = &line[0..2];
        let insertion = line.chars().last().unwrap();
        let mut chars = pair.chars();
        rules.insert(String::from(pair), (format!("{}{}", chars.next().unwrap(), insertion), format!("{}{}", insertion, chars.next().unwrap())));
    }
    //Create polymer
    for tuple in init_vals.chars().tuple_windows::<(char, char)>() {
        let tuple = polymer.entry(format!("{}{}", tuple.0, tuple.1)).or_insert(0);
        *tuple += 1;
    }
    //Create counter
    for c in init_vals.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    (polymer, counter, rules)
}

fn solve(iterations: u32) -> u128 {
    let (mut polymer, mut counter, rules) = get_input();

    //Insertions
    for _ in 0..iterations {
        let mut old_polymer = polymer.clone();
        for (key, (value1, value2)) in &rules {
            let val = old_polymer.entry(key.clone()).or_insert(0);
            let val = *val;

            let parent = polymer.entry(key.clone()).or_insert(0);
            *parent -= val;

            let child1 = polymer.entry(value1.clone()).or_insert(0);
            *child1 += val;

            let child2 = polymer.entry(value2.clone()).or_insert(0);
            *child2 += val;

            let count = counter.entry(value1.chars().last().unwrap()).or_insert(0);
            *count += val;
        }
    }

    let most_common = counter.iter().max_by_key(|e| e.1).unwrap();
    let least_common = counter.iter().min_by_key(|e| e.1).unwrap();
    most_common.1 - least_common.1
}