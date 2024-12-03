use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn calculate_similarity_score(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut right_count = HashMap::new();
    for &num in &right {
        *right_count.entry(num).or_insert(0) += 1;
    }

    // Step 2: Calculate similarity score
    let similarity_score: i32 = left.iter()
        .map(|&l| l * right_count.get(&l).unwrap_or(&0))
        .sum();

    similarity_score
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

            let result = calculate_similarity_score(left, right);
            println!("Similarity Score: {}", result);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
