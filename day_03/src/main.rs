mod helpers;

const WORD_SIZE: usize = 12;
const INPUT_FILE: &str = "input.txt";

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let words = helpers::get_input();
    let mut gamma = 0;
    let mut epsilon = 0;

    for char_index in 0..WORD_SIZE {
        if helpers::dominant_digits(&words, char_index, &vec![1; words.len()]) > 0 {
            //More ones than zeros
            gamma *= 2; //Multiplying by 2 means appedning a zero to a binary number
            gamma += 1; //Adding 1 after multiplying appends a 1 instead
            epsilon *= 2;
        } else {
            gamma *= 2;
            epsilon *= 2;
            epsilon += 1;
        }
    }

    println!("Part one answer: {}", gamma * epsilon);
}

fn part_two() {
    let words = helpers::get_input();

    //keep_one defines if a one should be kept. The inversion of keep_one will be applied to zeros.
    let oxygen_keep_one = |x| x >= 0;
    let oxygen_living_numbers = helpers::solve(oxygen_keep_one, &words);

    let co2_keep_one = |x| x < 0;
    let co2_living_numbers = helpers::solve(co2_keep_one, &words);

    //Find the last surviving element
    let oxygen = words.iter().nth(oxygen_living_numbers.iter().position(|&n| n == 1).unwrap()).unwrap();
    let co2 = words.iter().nth(co2_living_numbers.iter().position(|&n| n == 1).unwrap()).unwrap();

    //Convert last surviving string to number
    let oxygen = isize::from_str_radix(oxygen, 2).unwrap();
    let co2 = isize::from_str_radix(co2, 2).unwrap();

    println!("Part two answer: {}", oxygen * co2);
}