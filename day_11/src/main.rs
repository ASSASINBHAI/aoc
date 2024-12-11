use std::fs;
use std::collections::HashMap;

fn count(stone: u64, depth: usize, cache: &mut HashMap<(u64, usize), u64>) -> u64 {
    if let Some(&result) = cache.get(&(stone, depth)) {
        return result;
    }

    let result = if depth == 0 {
        1
    } else if stone == 0 {
        count(1, depth - 1, cache)
    } else {
        let num_digits = (stone as f64).log10().floor() as u32 + 1;
        
        if num_digits % 2 == 0 {
            let d = 10u64.pow(num_digits / 2);
            count(stone / d, depth - 1, cache) + count(stone % d, depth - 1, cache)
        } else {
            count(stone * 2024, depth - 1, cache)
        }
    };

    cache.insert((stone, depth), result);
    result
}

fn main() {
    // Read input from file
    let input = fs::read_to_string("input.txt")
        .expect("Unable to read file")
        .trim()
        .to_string();

    // Extract stones
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Memoization cache
    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();

    // Calculate results for depths 25 and 75
    let result_25: u64 = stones.iter().map(|&x| count(x, 25, &mut cache)).sum();
    let result_75: u64 = stones.iter().map(|&x| count(x, 75, &mut cache)).sum();

    println!("{}", result_25);
    println!("{}", result_75);
}