use std::fs;

const INPUT_FILE: &str = "input.txt";
const NBR_OF_DAYS: usize = 256;
const LIFETIME: u8 = 6;

fn main() {
    solve();
}

//The second number in a fish tuple is the "weight" of the fish, i.e how many normal fish it is counted as.
fn get_input() -> Vec<(u8, u64)> {
    let input = fs::read_to_string(INPUT_FILE)
        .expect("Failed to read file");

    input.split(',')
        .map(|n|
            (n.parse::<u8>().expect("Not a number"), 1)
        ).collect()
}

fn solve() {
    let mut fish = get_input();

    for _ in 0..NBR_OF_DAYS {
        decrease_timers(&mut fish);
    }

    let mut fish_count: u64 = 0;
    for f in fish {
        fish_count += f.1 as u64;
    }
    println!("{}", fish_count);

}

fn decrease_timers(fish: &mut Vec<(u8, u64)>) {
    let mut to_add: u64 = 0;
    for i in 0..fish.len() {
        if fish[i].0 == 0 {
            to_add += fish[i].1;
            fish[i].0 = LIFETIME;
        } else {
            fish[i].0 -= 1;
        }
    }

    fish.append(&mut vec![(LIFETIME + 2, to_add)]);
}