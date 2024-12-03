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

        if is_safe_report(&report) {
            safe_count += 1;
        }
    }

    println!("Number of safe reports: {}", safe_count);

    Ok(())
}