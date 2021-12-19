const INPUT_FILE: &str = "input.txt";

fn main() {
    part_one();
}

//Tuples consist of the value and a boolean flashed_this_turn
fn get_input() -> Vec<Vec<(u8, bool)>> {
    let input = std::fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file");
    
    input.lines().map(|line| {
        line.chars().map(|c| {
            (c.to_digit(10).unwrap() as u8, false)
        }).collect()
    }).collect()
}

fn part_one() {
    let mut grid = get_input();
    let mut total_flashes = 0;
    let mut has_won = false;
    for i in 1..500 {
        increase_all(&mut grid);
        loop {
            let flashes = flash(&mut grid);
            total_flashes += flashes;
            if flashes == 0 {break}
        }
        if i == 100 {
            println!("Answer part one: {}", total_flashes);
        }
        if !has_won && check_part_two(&grid) {
            println!("Answer part two: {}", i);
            has_won = true;
        }
        reset_has_flashed(&mut grid);
    }

}

fn increase_all(grid: &mut Vec<Vec<(u8, bool)>>) {
    for row in grid {
        for octopus in row {
            octopus.0 += 1;
        }
    }
}

fn flash(grid: &mut Vec<Vec<(u8, bool)>>) -> u32 {
    let mut flashes = 0;
    let grid_len = grid.iter().len();
    for row_index in 0..grid_len {
        for oct_index in 0..grid_len {
            let value = &mut grid[row_index][oct_index];
            if value.0 > 9 {
                value.1 = true;
                value.0 = 0;
                flashes += 1;
                increase_around(grid, row_index, oct_index);
            }
        }
    }
    flashes
}

fn check_part_two(grid: &Vec<Vec<(u8, bool)>>) -> bool{
    let mut has_won = true;
    for row in grid {
        for octopus in row {
            if octopus.1 == false {
                has_won = false;
            }
        }
    }
    has_won
}

fn increase_around(grid: &mut Vec<Vec<(u8, bool)>>, row_index: usize, oct_index: usize) {
    if row_index > 0 && !grid[row_index - 1][oct_index].1 {
        grid[row_index - 1][oct_index].0 += 1;
    }
    if oct_index > 0 && !grid[row_index][oct_index - 1].1 {
        grid[row_index][oct_index - 1].0 += 1;
    }
    if row_index < grid.len() - 1 && !grid[row_index + 1][oct_index].1 {
        grid[row_index + 1][oct_index].0 += 1;
    }
    if oct_index < grid.len() - 1 && !grid[row_index][oct_index + 1].1 {
        grid[row_index][oct_index + 1].0 += 1;
    }

    if row_index > 0 && oct_index > 0 && !grid[row_index - 1][oct_index - 1].1 {
        grid[row_index - 1][oct_index - 1].0 += 1;
    }
    if row_index > 0 && oct_index < grid.len() - 1 && !grid[row_index - 1][oct_index + 1].1 {
        grid[row_index - 1][oct_index + 1].0 += 1;
    }
    if row_index < grid.len() - 1 && oct_index > 0 && !grid[row_index + 1][oct_index - 1].1 {
        grid[row_index + 1][oct_index - 1].0 += 1;
    }
    if row_index < grid.len() - 1 && oct_index < grid.len() - 1 && !grid[row_index + 1][oct_index + 1].1 {
        grid[row_index + 1][oct_index + 1].0 += 1;
    }
}

fn reset_has_flashed(grid: &mut Vec<Vec<(u8, bool)>>) {
    let grid_len = grid.iter().len();
    for row_index in 0..grid_len {
        for oct_index in 0..grid_len {
            grid[row_index][oct_index].1 = false;
        }
    }
}