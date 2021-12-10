use std::fs;

fn main() {
    part_two();
}

fn list_of_input() -> Vec<i32> {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read file");
    
    let input = input.split_whitespace();

    input.map(|word| word.parse::<i32>().expect("Wrong input file. Something was not a number")).collect()
}

fn _part_one() {
    let numbers = list_of_input();
    let mut counter = 0;

    for window in numbers.windows(2) {
        if window[1] > window[0] {
            counter += 1;
        };
    }

    println!("{}", counter);
}

fn part_two() {
    let numbers = list_of_input();
    let mut counter = 0;

    for window in numbers.windows(4) {
        if (window[0] + window[1] + window[2]) < (window[1] + window[2] + window[3]) {
            counter += 1;
        };
    }

    println!("{}", counter);
}