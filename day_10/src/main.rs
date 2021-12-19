use std::collections::HashMap;

const INPUT_FILE: &str = "input.txt";
const OPENERS: [char; 4] = ['(', '[', '{', '<'];

fn main() {
    solve();
}

fn get_input() -> Vec<Vec<char>> {
    std::fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn solve() {
    let input = get_input();
    let mut illegal_char_score = 0;
    let mut incomplete_score;
    let mut corrupted;
    let mut incomplete_lines: Vec<u128> = vec![];

    let mut map = HashMap::new();
    map.insert(')', '(');
    map.insert(']', '[');
    map.insert('}', '{');
    map.insert('>', '<');

    for line in input {
        let mut stack: Vec<char> = vec![];
        corrupted = false;
        for &c in &line {
            if OPENERS.contains(&c) {
                stack.push(c);
            } else {
                if map.entry(c).or_default() == stack.last().unwrap() {
                    stack.pop();
                } else {
                    match c {
                        ')' => illegal_char_score += 3,
                        ']' => illegal_char_score += 57,
                        '}' => illegal_char_score += 1197,
                        '>' => illegal_char_score += 25137,
                        _ => ()
                    }
                    corrupted = true;
                    break;
                }
            }
        }
        if !corrupted {
            incomplete_score = 0;
            for c in stack.iter().rev() {
                incomplete_score *= 5;
                incomplete_score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0
                }
            }
            incomplete_lines.push(incomplete_score);
        }
    }

    println!("Answer part one: {}", illegal_char_score);
    incomplete_lines.sort();
    let middle = incomplete_lines.len() / 2;
    println!("Answer part two: {:?}", incomplete_lines[middle]);
}