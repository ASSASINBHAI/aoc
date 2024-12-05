use std::fs;

fn is_update_correct(update: &Vec<i32>, rules: &Vec<String>) -> bool {
    for rule in rules {
        let parts: Vec<&str> = rule.split('|').collect();
        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[1].parse().unwrap();
        
        if update.contains(&x) && update.contains(&y) {
            let index_x = update.iter().position(|&v| v == x).unwrap();
            let index_y = update.iter().position(|&v| v == y).unwrap();
            if index_x > index_y {
                return false;
            }
        }
    }
    true
}

fn find_middle_page_number(update: &Vec<i32>) -> i32 {
    update[update.len() / 2]
}

fn read_input(file_path: &str) -> (Vec<String>, Vec<Vec<i32>>) {
    let content = fs::read_to_string(file_path).unwrap();
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut found_separator = false;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() {
            found_separator = true;
            continue;
        }
        
        if !found_separator {
            rules.push(line.to_string());
        } else {
            let update: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
            updates.push(update);
        }
    }

    (rules, updates)
}

fn part_1(rules: &Vec<String>, updates: &Vec<Vec<i32>>) -> i32 {
    let correct_updates: Vec<&Vec<i32>> = updates.iter()
        .filter(|update| is_update_correct(update, &rules))
        .collect();

    correct_updates.iter()
        .map(|update| find_middle_page_number(update))
        .sum()
}

fn main() {
    let (rules, updates) = read_input("input.txt");
    
    // Part 1: Calculate the result
    let part_1_result = part_1(&rules, &updates);

    // Print result for Part 1
    println!("{}", part_1_result);
}
