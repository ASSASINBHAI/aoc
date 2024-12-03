use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn is_safe_report(report: &Vec<i32>) -> bool {
    // Check if all adjacent differences are between 1 and 3
    let differences: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();
    if !differences.iter().all(|&diff| diff.abs() >= 1 && diff.abs() <= 3) {
        return false;
    }

    // Check if the sequence is monotonic (either all increasing or all decreasing)
    let all_increasing = differences.iter().all(|&diff| diff > 0);
    let all_decreasing = differences.iter().all(|&diff| diff < 0);
    all_increasing || all_decreasing
}

fn can_be_safe_with_one_removal(report: &Vec<i32>) -> bool {
    // Try removing each element and check if the result is safe
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        if is_safe_report(&new_report) {
            return true;
        }
    }
    false
}

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;
        let report: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Check if the report is safe either directly or by removing one level
        if is_safe_report(&report) || can_be_safe_with_one_removal(&report) {
            safe_count += 1;
        }
    }

    println!("Number of safe reports: {}", safe_count);

    Ok(())
}
