use crate::BOARD_SIZE;

#[derive(Debug)]
pub struct Board {
    values: Vec<Vec<u8>>,
    filled_in: [[bool; BOARD_SIZE]; BOARD_SIZE],
    filled_in_count: u8,
    has_won: bool
}

impl Board {
    pub fn new(values: Vec<Vec<u8>>) -> Board {
        Board {
            values,
            filled_in: [[false; BOARD_SIZE]; BOARD_SIZE],
            filled_in_count: 0,
            has_won: false
        }
    }

    pub fn fill_in_index(&mut self, index: usize) {
        self.filled_in[index / BOARD_SIZE][index % BOARD_SIZE] = true;
        self.filled_in_count += 1;
    }

    pub fn fill_in(&mut self, num: &u8) -> bool {
        let mut has_won = false;
        for (index, sqr) in self.values.clone().iter().flatten().enumerate() {
            if sqr == num {
                self.fill_in_index(index);
                if self.check_win(index, *sqr) {
                    has_won = true;
                }
            }
        };
        has_won
    }

    pub fn check_win(&mut self, sqr_index: usize, square: u8) -> bool {
        let mut row_won = true;
        let mut col_won = true;
        for i in 0..BOARD_SIZE {
            if !self.filled_in[i][sqr_index % BOARD_SIZE] {
                row_won = false;
            }
            if !self.filled_in[sqr_index / BOARD_SIZE][i] {
                col_won = false;
            }
        }
        let won = row_won || col_won;

        //Remove this has_won-check for part 1
        if won && !self.has_won {
            println!("{:?} has won!", self);
            self.calc_points(sqr_index, square);
            self.has_won = true;
        }

        row_won || col_won
    }

    pub fn calc_points(&self, sqr_index: usize, square: u8) {
        let mut sum = 0;
        let values = self.values.iter().flatten();

        for (index, sqr) in values.enumerate() {
            if !self.filled_in[index / BOARD_SIZE][index % BOARD_SIZE] {
                sum += *sqr as u32;
            }
        }

        println!("Sum: {}", sum);
        println!("Last square: {}", square);
        println!("Answer: {}", sum as usize * square as usize);
    }
}