use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn main() {
    let grid = read_input("input.txt");
    let first_result = first_part(&grid);
    let second_result = second_part(&grid);

    println!("First part result: {}", first_result);
    println!("Second part result: {}", second_result);
}

fn read_input(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect()
}

fn get_neighbors(x: usize, y: usize, grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let directions = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
    let mut neighbors = Vec::new();
    let height = grid.len();
    let width = grid[0].len();

    for &(dx, dy) in &directions {
        let new_x = x.wrapping_add(dx);
        let new_y = y.wrapping_add(dy);

        if new_x < height && new_y < width {
            neighbors.push((new_x, new_y));
        }
    }

    neighbors
}

fn find_trailheads(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 0 {
                trailheads.push((i, j));
            }
        }
    }

    trailheads
}

fn find_reachable_nines(start_x: usize, start_y: usize, grid: &[Vec<u8>]) -> usize {
    let mut visited = HashSet::new();
    let mut reachable_nines = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y, 0));

    while let Some((x, y, current_height)) = queue.pop_front() {
        if !visited.insert((x, y)) {
            continue;
        }

        if grid[x][y] == 9 {
            reachable_nines.insert((x, y));
            continue;
        }

        for (next_x, next_y) in get_neighbors(x, y, grid) {
            if grid[next_x][next_y] == current_height + 1 {
                queue.push_back((next_x, next_y, current_height + 1));
            }
        }
    }

    reachable_nines.len()
}

fn count_paths(
    start_x: usize,
    start_y: usize,
    grid: &[Vec<u8>],
    memo: &mut HashMap<(usize, usize, u8), usize>,
) -> usize {
    let current_height = grid[start_x][start_y];

    if grid[start_x][start_y] == 9 {
        return 1;
    }

    if let Some(&cached_result) = memo.get(&(start_x, start_y, current_height)) {
        return cached_result;
    }

    let mut total_paths = 0;
    for (next_x, next_y) in get_neighbors(start_x, start_y, grid) {
        if grid[next_x][next_y] == current_height + 1 {
            total_paths += count_paths(next_x, next_y, grid, memo);
        }
    }

    memo.insert((start_x, start_y, current_height), total_paths);
    total_paths
}

fn first_part(grid: &[Vec<u8>]) -> usize {
    let mut total_score = 0;
    let trailheads = find_trailheads(grid);

    for (x, y) in trailheads {
        total_score += find_reachable_nines(x, y, grid);
    }

    total_score
}

fn second_part(grid: &[Vec<u8>]) -> usize {
    let mut total = 0;
    let trailheads = find_trailheads(grid);
    let mut memo = HashMap::new();

    for (x, y) in trailheads {
        total += count_paths(x, y, grid, &mut memo);
    }

    total
}
