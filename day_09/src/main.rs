const INPUT_FILE: &str = "input.txt";

//This is easily the ugliest solution yet
fn main() {
    part_one();
    part_two();
}

fn get_input() -> Vec<Vec<u8>> {
    let input = std::fs::read_to_string(INPUT_FILE).expect("Failed to read file");

    input.lines().map(|line| {
        line.chars().map(|c| {
            c.to_digit(10).unwrap() as u8
        }).collect()
    }).collect()
}

fn part_one() {
    let input = get_input();
    let mut total_risk = 0u32;

    for (row, line) in input.iter().enumerate() {
        for (col, num) in line.iter().enumerate() {
            if is_low_point(&input, row, col, *num) {
                total_risk += 1 + *num as u32;
            }
        }
    }

    println!("Answer part one: {}", total_risk);
}

fn part_two() {
    let input = get_input();
    let mut basins: Vec<Vec<(usize, usize)>> = Default::default();

    for (row, line) in input.iter().enumerate() {
        for (col, node) in line.iter().enumerate() {
            if *node != 9 {
                add_to_basins(&input, &mut basins, row, col);
            }
        }
    }

    let mut sizes: Vec<usize> = basins.iter().map(|v| v.len()).collect();
    sizes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top_sizes: usize = sizes.iter().take(3).product();
    println!("Answer part two: {}", top_sizes);
}

fn add_to_basins(input: &Vec<Vec<u8>>, basins: &mut Vec<Vec<(usize, usize)>>, row: usize, col: usize) {
    if row > 0 && input[row - 1][col] != 9 {
        if col > 0 && input[row][col - 1] != 9 {
            if let Some(above_basin_index) = find_basin_containing(&basins, row - 1, col) {
                if let Some(left_basin_index) = find_basin_containing(&basins, row, col - 1) {
                    if above_basin_index != left_basin_index {
                        let left_basin = &basins[left_basin_index].clone();
                        basins.remove(left_basin_index);
                        let above_basin_index = find_basin_containing(&basins, row - 1, col).unwrap();
                        let above_basin = &mut basins[above_basin_index];
                        above_basin.extend(left_basin);
                        above_basin.push((row, col));
                    } else {
                        let above_basin = &mut basins[above_basin_index];
                        above_basin.push((row, col));
                    }
                }
            }
        } else {
            if let Some(above_basin_index) = find_basin_containing(&basins, row - 1, col) {
                basins[above_basin_index].push((row, col));
            }
        }
    } else if col > 0 && input[row][col - 1] != 9 {
        if let Some(left_basin_index) = find_basin_containing(&basins, row, col - 1) {
            basins[left_basin_index].push((row, col));
        }
    } else {
        let v = vec![(row, col)];
        basins.push(v);
    }
}

///Returns index of HashMap containing given element
fn find_basin_containing(basins: &Vec<Vec<(usize, usize)>>, row: usize, col: usize) -> Option<usize> {
    for (index, basin) in basins.iter().enumerate() {
        if basin.contains(&(row, col)) {
            return Some(index);
        }
    };
    None
}

fn is_low_point(input: &Vec<Vec<u8>>, row: usize, col: usize, num: u8) -> bool {
    if row > 0 && num >= input[row - 1][col] {
        return false;
    } else if row < input.iter().len() - 1 && num >= input[row + 1][col] {
        return false;
    } else if col > 0 && num >= input[row][col - 1] {
        return false;
    } else if col < input[0].iter().len() - 1 && num >= input[row][col + 1] {
        return false;
    }
    true
}