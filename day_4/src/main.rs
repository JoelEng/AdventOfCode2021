use std::fs;
use board::Board;
use itertools::Itertools;

mod board;

const INPUT_FILE: &str = "input.txt";
const BOARD_SIZE: usize = 5;

fn main() {
    part_one();
}

fn get_input() -> (Vec<u8>, Vec<board::Board>) {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file");
    let mut input = input.lines().filter(|&x| !x.is_empty());

    //Split drawn numbers
    let draws = input.next().unwrap().split(',');
    //Parse drawn numbers to u8
    let draws: Vec<u8> = draws.map(|x| x.parse().expect("Something went wrong")).collect();

    //Make an array of Board structs
    let boards: Vec<Board> = input.chunks(BOARD_SIZE).into_iter()
        .map(|board|
            Board::new(board.map(|row|
                row.split_whitespace().map(|num|
                    num.parse().unwrap()
                ).collect()
            ).collect())
        ).collect(); //There has to be a better way to do this

    (draws, boards)
}

fn part_one() {
    let (draws, mut boards) = get_input();

    //Use this code for part 2
    for draw in draws {
        for board in &mut boards {
            board.fill_in(&draw);
        }
    };

    //Use this code for part 1
    /*'outer:for draw in draws {
        for board in &mut boards {
            if board.fill_in(&draw) {
                break 'outer;
            }
        }
    };*/

}