use std::fs;

const INPUT_FILE: &str = "input.txt";
const NBR_OF_DAYS: usize = 951; //Change this for parts 1 and 2. 951 is the biggest that a u128 can handle
const ARRAY_SIZE: usize = 9;

fn main() {
    solve();
}

fn get_input() -> [u128; ARRAY_SIZE] {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file");

    let mut fish = [0; ARRAY_SIZE];
    for i in input.split(',') {
        fish[i.parse::<usize>().expect("Not a valid number")] += 1;
    };
    fish
}

fn solve() {
    let mut fish = get_input();
    for _ in 0..NBR_OF_DAYS {
        decrease_timers(&mut fish);
    }
    println!("{}", fish.iter().sum::<u128>());
}

fn decrease_timers(fish: &mut [u128; ARRAY_SIZE]) {
    let fish0 = fish[0];
    for i in 0..(ARRAY_SIZE - 1) {
        fish[i] = fish[i + 1];
    }
    fish[ARRAY_SIZE - 1] = fish0;
    fish[ARRAY_SIZE - 3] += fish0;
}