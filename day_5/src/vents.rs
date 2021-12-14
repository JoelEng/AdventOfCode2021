use crate::Board;
use crate::IS_PART_TWO;
use std::cmp::{min, max};

#[derive(Debug)]
pub struct Vent {
    start: (usize, usize),
    end: (usize, usize)
}

impl Vent {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Vent {
        Vent {
            start,
            end
        }
    }

    pub fn add_to(self, board: &mut Board) {
        let x1 = self.start.0;
        let y1 = self.start.1;

        let x2 = self.end.0;
        let y2 = self.end.1;

        if x1 == x2 {
            let add_to_board = |y: usize| board[x1][y] += 1;
            Vent::helper(y1, y2, add_to_board);
        } else if y1 == y2 {
            let add_to_board = |x: usize| board[x][y1] += 1;
            Vent::helper(x1, x2, add_to_board);
        } else if (x2 as isize - x1 as isize).abs() == (y2 as isize - y1 as isize).abs() && IS_PART_TWO {
            let mut add_to_board = |x: usize, y: usize| board[x][y] += 1;
            let x_start = min(x1, x2);
            let x_end = max(x1, x2);
            let y_start = if x_start == x1 {y1} else {y2};
            let y_end = if !(x_start == x1) {y1} else {y2};
            //let x_range = x_start..=x_end;
            let len = x_end - x_start;

            if y_start < y_end {
                for l in 0..=len {
                    add_to_board(x_start + l, y_start + l);
                }
            } else {
                for l in 0..=len {
                    add_to_board(x_start + l, (y_start as isize - l as isize) as usize);
                }
            }


            /*if y_start < y_end {
                let y_range = y_start..=y_end;

                for (x, y) in x_range.zip(y_range) {
                    add_to_board(x, y as usize);
                }
            } else {
                let y_range = (y_start..=y_end).rev();
                println!("{} {}, {:?}", y_start, y_end, y_range);
                for (x, y) in x_range.zip(y_range) {
                    add_to_board(x, y);
                }
            }*/
        }
    }

    pub fn helper<F>(a1: usize, a2: usize, mut add_to_board: F)
        where F: FnMut(usize) -> () {
        let start = min(a1, a2);
        let end = max(a1, a2);
        for y in start..=end {
            add_to_board(y);
        }
    }
}