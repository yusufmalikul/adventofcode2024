use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // by @yusufmalikul
    let file = File::open("input.txt").expect("Cannot open file");

    let reader = io::BufReader::new(file);

    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    let mut diff = 0;

    for line in reader.lines() {
        let line = line.expect("Cannot read line");

        // split
        let parts: Vec<&str> = line.split_whitespace().collect();
        let a: i32 = parts[0].parse().expect("cannot parse first integer");
        let b: i32 = parts[1].parse().expect("cannot parse second integer");

        first_numbers.push(a);
        second_numbers.push(b);
    }

    first_numbers.sort();
    second_numbers.sort();

    for (a, b) in first_numbers.iter().zip(second_numbers.iter()) {
        diff = diff + (a-b).abs();
    }

    println!("{}", diff);
}
