use std::fs;

fn solve(part: u32) {
    let add = if part == 2 { 10_000_000_000_000 } else { 0 };
    let mut tokens = 0;

    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let lines: Vec<&str> = input.lines().collect();

    let mut x1 = 0;
    let mut y1 = 0;
    let mut x2 = 0;
    let mut y2 = 0;

    for line in lines {
        if line.starts_with("Button") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let a = parts[1].split(':').next().unwrap();
            if a == "A" {
                x1 = parts[2][2..parts[2].len() - 1].parse::<i64>().unwrap();
                y1 = parts[3][2..].parse::<i64>().unwrap();
            } else {
                x2 = parts[2][2..parts[2].len() - 1].parse::<i64>().unwrap();
                y2 = parts[3][2..].parse::<i64>().unwrap();
            }
        } else if line.starts_with("Prize") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let c = parts[1][2..parts[1].len() - 1].parse::<i64>().unwrap() + add;
            let d = parts[2][2..].parse::<i64>().unwrap() + add;

            let denominator = x1 * y2 - y1 * x2;
            if denominator == 0 {
                continue;
            }

            let a = (c * y2 - d * x2) as f64 / denominator as f64;
            let b = (d * x1 - c * y1) as f64 / denominator as f64;

            if a.fract() == 0.0 && b.fract() == 0.0 {
                tokens += 3 * a as i64 + b as i64;
            }
        }
    }

    println!("{}", tokens);
}

fn main() {
    solve(1);
    solve(2);
}
