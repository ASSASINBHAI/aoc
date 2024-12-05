use std::collections::{HashMap, VecDeque};
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

fn topological_sort(update: &Vec<i32>, rules: &Vec<String>) -> Vec<i32> {
    let mut indegree: HashMap<i32, i32> = HashMap::new();
    let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();
    
    for rule in rules {
        let parts: Vec<&str> = rule.split('|').collect();
        let x: i32 = parts[0].parse().unwrap();
        let y: i32 = parts[1].parse().unwrap();

        if update.contains(&x) && update.contains(&y) {
            adjacency_list.entry(x).or_insert(Vec::new()).push(y);
            *indegree.entry(y).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<i32> = VecDeque::new();
    let mut sorted_pages = Vec::new();

    for &page in update {
        if *indegree.get(&page).unwrap_or(&0) == 0 {
            queue.push_back(page);
        }
    }

    while let Some(page) = queue.pop_front() {
        sorted_pages.push(page);
        if let Some(neighbors) = adjacency_list.get(&page) {
            for &neighbor in neighbors {
                let count = indegree.entry(neighbor).or_insert(0);
                *count -= 1;
                if *count == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    sorted_pages
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

fn part_2(rules: &Vec<String>, updates: &Vec<Vec<i32>>) -> i32 {
    let incorrect_updates: Vec<&Vec<i32>> = updates.iter()
        .filter(|update| !is_update_correct(update, &rules))
        .collect();

    incorrect_updates.iter()
        .map(|update| {
            let sorted_update = topological_sort(update, &rules);
            find_middle_page_number(&sorted_update)
        })
        .sum()
}

fn main() {
    let (rules, updates) = read_input("input.txt");
    
    // Part 2: Calculate the result
    let part_2_result = part_2(&rules, &updates);

    // Print result for Part 2
    println!("{}", part_2_result);
}
