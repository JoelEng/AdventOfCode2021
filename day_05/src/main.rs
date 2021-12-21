use std::fs;
use crate::vents::Vent;

mod vents;

const INPUT_FILE: &str = "input.txt";
const BOARD_SIZE: usize = 1000;
type Board = [[u32; BOARD_SIZE]; BOARD_SIZE];
const IS_PART_TWO: bool = true;

fn main() {
    //Change const IS_PART_TWO to switch part
    solve();
}

fn get_input() -> Vec<Vent> {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file");

    input.lines().map(|line| {
        let mut line = line.split_whitespace();
        let first = create_tuple(line.next());
        let second = create_tuple(line.last());
        Vent::new(first, second)
    }).collect::<Vec<Vent>>()
}

///Helper function for get_input
fn create_tuple(input: Option<&str>) -> (usize, usize) {
    let mut elems = input.unwrap().trim().split(',');
    (elems.next().unwrap().parse::<usize>().expect("Something went wrong"), elems.next().unwrap().parse::<usize>().expect("Something went wrong"))
}

fn solve() {
    let vents = get_input();
    let mut board: Board = [[0; BOARD_SIZE]; BOARD_SIZE];

    for vent in vents {
        vent.add_to(&mut board);
    }

    let mut count = 0;
    for sqr in board.iter().flatten() {
        if *sqr >= 2 {
            count += 1;
        }
    }
    println!("{}", count);
}