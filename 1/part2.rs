use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // by @yusufmalikul
    let file = File::open("input.txt").expect("Cannot open file");

    let reader = io::BufReader::new(file);

    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    let mut score = 0;

    for line in reader.lines() {
        let line = line.expect("Cannot read line");

        // split
        let parts: Vec<&str> = line.split_whitespace().collect();
        let a: i32 = parts[0].parse().expect("cannot parse first integer");
        let b: i32 = parts[1].parse().expect("cannot parse second integer");

        first_numbers.push(a);
        second_numbers.push(b);
    }

    for &first_number in &first_numbers {
        let count = second_numbers.iter().filter(|&&num| num == first_number).count();
        if count > 0 {
            score = score + (first_number * count as i32);
        }
    }

    println!("similarity score: {}", score);

}
