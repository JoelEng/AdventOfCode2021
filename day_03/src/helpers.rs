use std::fs;

use crate::{WORD_SIZE, INPUT_FILE};

pub fn get_input() -> Vec<String> {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file");
  
    input.lines().map(|word| String::from(word)).collect()
}

///Returns positive number if more ones than zeros on the specified index
pub fn dominant_digits(words: &Vec<String>, char_index: usize, living_numbers: &Vec<u32>) -> i32 {
    let mut sum = 0;

    for (word_index, word) in words.iter().enumerate() {
        match word.chars().nth(char_index) {
            Some('1') => {
                if living_numbers[word_index] == 1 {
                    sum += 1;
                }
            },
            Some('0') => {
                if living_numbers[word_index] == 1 {
                    sum -= 1;
                }
            },
            _ => println!("Something went wrong")
        }
    }

    sum
}

pub fn solve<F>(keep_one: F, words: &Vec<String>) -> Vec<u32>
    where F: Fn(i32) -> bool {
    let mut living_numbers = vec![1; words.len()];

    'outer:for char_index in 0..WORD_SIZE {
        let keep_one = keep_one(dominant_digits(words, char_index, &living_numbers));
        for (word_index, word) in words.iter().enumerate() {
            if living_numbers.iter().sum::<u32>() == 1 {
                break 'outer;
            }
            match word.chars().nth(char_index) {
                Some(n) => {
                    if living_numbers[word_index] == 1 {
                        if n == '1' && keep_one || n == '0' && !keep_one {
                            //Keep the element
                        } else {
                            living_numbers[word_index] = 0;
                        }
                    }
                },
                None => println!("Something went wrong"),
            }
        }
    }

    living_numbers
}