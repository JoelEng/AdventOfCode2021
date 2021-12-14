use std::fs;

const INPUT_FILE: &str = "input.txt";

fn main() {
    part_one();
    part_two();
}

fn get_input() -> Vec<u32> {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file");

    input.split(',').map(|n| n.parse().expect("Not a number")).collect()
}

fn part_one() {
    let mut positions = get_input();
    let median = median(&mut positions);

    let mut fuel_use = 0;
    for pos in &positions {
        fuel_use += (*pos as i64 - median as i64).abs();
    }
    
    println!("{}", fuel_use);
}

fn part_two() {
    let positions = get_input();

    //height of the vec, i.e the largest number in it
    let height = positions.iter().max().unwrap();

    let mut min_fuel: i64 = 10000000000;
    for middle in 0..*height {
        let mut fuel_use = 0;
        for pos in &positions {
            let distance = (*pos as i64 - middle as i64).abs();
            fuel_use += distance * (distance + 1) / 2;
        }
        if fuel_use < min_fuel {
            min_fuel = fuel_use;
        }
    }
    
    println!("{}", min_fuel);
}

fn median(vec: &mut Vec<u32>) -> u32 {
    vec.sort();
    let middle = vec.len() / 2;
    vec[middle]
}