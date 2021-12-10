use std::fs;

fn main() {
    part_one();
    part_two();
}

fn get_input() -> Vec<(char, u32)> {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read file");
    
    let input = input.split('\n');

    input.map(|word| (word.chars().next().unwrap(), word.chars().last().unwrap().to_digit(10).unwrap())).collect()
}

fn part_one() {
    let input = get_input();
    let mut forward = 0;
    let mut depth = 0;

    for (direction, steps) in input {
        match direction {
            'f' => forward += steps,
            'u' => depth -= steps,
            'd' => depth += steps,
            _ => ()
        }
    }

    println!("{}", forward * depth);
}

fn part_two() {
    let input = get_input();
    let mut forward = 0;
    let mut depth = 0;
    let mut aim: i32 = 0;

    for (direction, steps) in input {
        let steps = steps as i32;

        match direction {
            'f' => {
                forward += steps;
                depth += aim * steps;
            },
            'u' => {
                aim -= steps;
            },
            'd' => {
                aim += steps;
            },
            _ => ()
        }
    }

    println!("{}", forward as i32 * depth);
}