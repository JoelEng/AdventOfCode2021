use std::collections::HashMap;
use itertools::Itertools;

const INPUT_FILE: &str = "input_example.txt";

fn main() {
    println!("Answer part one: {}", solve(10));
    //println!("Answer part two: {}", solve(40));
}

fn get_input() -> (String, HashMap<String, char>) {
    let input = std::fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file");
    let mut lines = input.lines();
    
    let mut rules: HashMap<String, char> = HashMap::new();
    let polymer = lines.next().unwrap().to_string();

    lines.next(); //Remove blank line
    for line in lines {
        let pair = line[0..2].to_string();
        let insertion = line.chars().last().unwrap();
        rules.insert(pair, insertion);
    }
    (polymer, rules)
}

fn solve(iterations: u32) -> u32 {
    let (polymer, mut rules) = get_input();
    let mut pol = polymer;
    
    //Insertions
    for i in 0..iterations {
        println!("{}", i);
        let pol_clone = pol.clone();
        let windows = pol_clone.chars().tuple_windows::<(char, char)>();
        pol = String::from(pol.chars().next().unwrap());

        for tuple in windows {
            let insertion = rules.entry(format!("{}{}", tuple.0, tuple.1)).or_default();
            pol = pol + &format!("{}{}", insertion, tuple.1);
        }
    }

    //Count chars
    let mut counter: HashMap<char, u32> = HashMap::new();
    for c in pol.chars() {
        let val = *counter.entry(c).or_insert(0);
        counter.insert(c, val + 1);
    }

    //Find most and least common char
    let most_common = counter.iter().max_by_key(|e| e.1).unwrap();
    let least_common = counter.iter().min_by_key(|e| e.1).unwrap();
    println!("Most common: {:?}", most_common);
    println!("Least common: {:?}", least_common);
    most_common.1 - least_common.1
}