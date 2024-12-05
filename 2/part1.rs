use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // by @yusufmalikul
    let file = File::open("input.txt").expect("Cannot open file");

    let reader = io::BufReader::new(file);

    let mut totalSafe = 0;
    for line in reader.lines() {
        let line = line.expect("Cannot read line");

        // Split and parse numbers into a vector
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|part| part.parse::<i32>().expect("Cannot parse number"))
            .collect();

        let mut largest_diff = 0;

        let mut level = "";
        let mut safe = true;
        for pair in numbers.windows(2) {
            let diff = (pair[0] - pair[1]);
            if (diff == 0 || diff.abs() > 3) {
                safe = false;
                break;
            }

            if (level == "") {
                if (diff > 0) {
                    level = "decreasing";
                } else {
                    level = "increasing";
                }
            } else {
                if (diff > 0 && level == "increasing") {
                    safe = false;
                    break;
                }

                if (diff < 0 && level == "decreasing") {
                    safe = false;
                    break;
                }

            }
        }

        if safe {
            totalSafe = totalSafe + 1;
            println!("{:?} this report safe {}", numbers, level);
        }
    }

    println!("total safe {}", totalSafe);
}
