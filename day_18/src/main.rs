use std::collections::{HashSet, VecDeque};
use std::fs;

type Point = (i32, i32);

fn parse_input(filename: &str) -> Vec<Point> {
    let content = fs::read_to_string(filename).expect("Failed to read input file");
    content
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line
                .trim()
                .split(',')
                .map(|x| x.parse().expect("Invalid number"))
                .collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn bfs(width: i32, open_set: &HashSet<Point>, wall_set: &HashSet<Point>) -> Option<(i32, Vec<Point>)> {
    let start_point = (0, 0);
    let end_point = (width, width);
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();
    frontier.push_back((0, start_point, Vec::new()));

    while let Some((distance, location, history)) = frontier.pop_front() {
        if location == end_point {
            return Some((distance, history));
        }
        if visited.contains(&location) {
            continue;
        }
        visited.insert(location);

        let mut new_history = history.clone();
        new_history.push(location);

        for &(dx, dy) in &directions {
            let new_location = (location.0 + dx, location.1 + dy);
            if wall_set.contains(&new_location) || !open_set.contains(&new_location) || visited.contains(&new_location) {
                continue;
            }
            frontier.push_back((distance + 1, new_location, new_history.clone()));
        }
    }
    None
}

fn main() {
    let byte_list = parse_input("input.txt");

    let width = 70;
    let mut open_set: HashSet<Point> = (0..=width)
        .flat_map(|x| (0..=width).map(move |y| (x, y)))
        .collect();

    let mut wall_set = HashSet::new();
    for &(x, y) in &byte_list[..1024] {
        open_set.remove(&(x, y));
        wall_set.insert((x, y));
    }

    let part1_result = bfs(width, &open_set, &wall_set);
    let (part1_answer, mut first_history) = match part1_result {
        Some((distance, history)) => (distance, history.into_iter().collect::<HashSet<_>>()),
        None => {
            println!("No path found for Part 1.");
            return;
        }
    };

    let mut part2_answer = None;
    for (r, &new_byte) in byte_list.iter().enumerate().skip(1024) {
        open_set.remove(&new_byte);
        wall_set.insert(new_byte);

        if !first_history.contains(&new_byte) {
            continue;
        }

        println!("BFS performed on byte {}, {:?}", r, new_byte);
        let result = bfs(width, &open_set, &wall_set);
        if let Some((_, new_history)) = result {
            first_history.clear();
            first_history.extend(new_history);
        } else {
            part2_answer = Some(new_byte);
            break;
        }
    }

    println!("Part 1 Answer: {}", part1_answer);
    match part2_answer {
        Some((x, y)) => println!("Part 2 Answer: {},{}", x, y),
        None => println!("Part 2 Answer could not be determined."),
    }
}
