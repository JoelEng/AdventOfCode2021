use once_cell::sync::Lazy;
use regex::Regex;
use std::process::Command;

static MS_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Time: (\d+)ms").unwrap());

fn extract_time(s: &str) -> Option<u32> {
    let capture = MS_REGEX.captures_iter(s).next()?;
    capture[1].parse().ok()
}

fn main() {
    let mut total_time = 0;
    let mut got_to = 25;
    for day_num in 1..=25 {
        let day = format!("{:0>2}", day_num);
        let cmd = Command::new("cargo")
            .args(&["run", "--release", "--bin", &day])
            .output()
            .unwrap();
        let output = String::from_utf8(cmd.stdout).unwrap();
        total_time += match extract_time(&output) {
            Some(x) => x,
            None => {
                got_to = day_num - 1;
                break;
            }
        };
        println!("\x1b[4;1mDay {}:\n\x1b[0m{}", day, output);
    }
    print!("\x1b[4;1m");
    if got_to == 25 {
        println!(
            "🎄 All days completed! 🎄 Total time: {}ms\x1b[0m",
            total_time
        );
    } else {
        println!(
            "{} days completed. Total time: {}ms\x1b[0m",
            got_to, total_time
        );
    }
}
