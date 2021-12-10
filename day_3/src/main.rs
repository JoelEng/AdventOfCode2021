use std::fs;

fn main() {
    part_one();
}

fn get_input() -> Vec<String> {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read file");
    
    input.lines().map(|word| String::from(word)).collect()
}

fn part_one() {
    let words = get_input();
    let mut array_one = [0; 12];
    let mut array_zero = [0; 12];

    for word in words {
        for i in 0..12 {
            match word.chars().nth(i) {
                Some('1') => array_one[i] += 1,
                Some('0') => array_zero[i] += 1,
                Some(_) => println!("Something went wrong. SOME(_)!"),
                None => println!("Something went wrong. NONE!"),
            }
        }
    }

    for i in 0..12 {
        println!("{}", array_one[i] - array_zero[i]);
    } //101111001110 //010000110001

    println!("{:?} {:?}", array_zero, array_one);
}
