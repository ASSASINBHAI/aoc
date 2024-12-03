use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn calculate_total_distance(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();
    left_sorted.sort();
    right_sorted.sort();

    let total_distance: i32 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    total_distance
}

fn read_input_from_file(filename: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() == 2 {
            left.push(numbers[0]);
            right.push(numbers[1]);
        }
    }

    Ok((left, right))
}

fn main() {
    match read_input_from_file("input.txt") {
        Ok((left, right)) => {
            let result = calculate_total_distance(left, right);
            println!("Total Distance: {}", result);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
