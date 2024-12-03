use std::fs;
use regex::Regex;

fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("Unable to read file")
}

fn part1(file: &str) -> i32 {
    let data = read_file(file);
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result = 0;
    
    for capture in re.captures_iter(&data) {
        let num1: i32 = capture[1].parse().unwrap();
        let num2: i32 = capture[2].parse().unwrap();
        result += num1 * num2;
    }
    
    result
}

fn part2(file: &str) -> i32 {
    let data = read_file(file);
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)").unwrap();
    let mut result = 0;
    let mut enabled = true;
    
    for capture in re.captures_iter(&data) {
        if let Some(_) = capture.get(1) {
            if enabled {
                let num1: i32 = capture[1].parse().unwrap();
                let num2: i32 = capture[2].parse().unwrap();
                result += num1 * num2;
            }
        } else if let Some(_) = capture.get(3) {
            enabled = true;
        } else if let Some(_) = capture.get(4) {
            enabled = false;
        }
    }
    
    result
}

fn main() {
    // Output for part 1 and part 2
    println!("Part 1: {}", part1("input.txt"));
    println!("Part 2: {}", part2("input.txt"));
}
