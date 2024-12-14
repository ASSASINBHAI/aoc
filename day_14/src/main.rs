use std::fs::File;
use std::io::{self, BufRead};
use std::collections::VecDeque;

#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn parse_input(file_path: &str) -> io::Result<Vec<Robot>> {
    let mut robots = Vec::new();

    let file = File::open(file_path)?;
    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let position = parts[0][2..]
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let velocity = parts[1][2..]
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            robots.push(Robot {
                position: (position[0], position[1]),
                velocity: (velocity[0], velocity[1]),
            });
        }
    }

    Ok(robots)
}

fn simulate_motion(robots: &[Robot], width: i32, height: i32, seconds: i32) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; width as usize]; height as usize];

    for robot in robots {
        let x = (robot.position.0 + robot.velocity.0 * seconds).rem_euclid(width);
        let y = (robot.position.1 + robot.velocity.1 * seconds).rem_euclid(height);
        grid[y as usize][x as usize] += 1;
    }

    grid
}

fn calculate_safety_factor(grid: &[Vec<i32>]) -> i32 {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrants = [0; 4]; // Top-left, Top-right, Bottom-left, Bottom-right

    for y in 0..height {
        for x in 0..width {
            if x == mid_x || y == mid_y {
                continue;
            }

            if x < mid_x && y < mid_y {
                quadrants[0] += grid[y as usize][x as usize];
            } else if x >= mid_x && y < mid_y {
                quadrants[1] += grid[y as usize][x as usize];
            } else if x < mid_x && y >= mid_y {
                quadrants[2] += grid[y as usize][x as usize];
            } else if x >= mid_x && y >= mid_y {
                quadrants[3] += grid[y as usize][x as usize];
            }
        }
    }

    quadrants.iter().product()
}

fn time_to_cluster(robots: &[Robot], width: i32, height: i32) -> i32 {
    let mut t = 1;
    let mut dt = 1;
    let mut iterations = VecDeque::new();

    loop {
        let mut guard_x = Vec::new();
        let mut guard_y = Vec::new();

        for robot in robots {
            let (x, y) = (
                (robot.position.0 + robot.velocity.0 * t).rem_euclid(width),
                (robot.position.1 + robot.velocity.1 * t).rem_euclid(height),
            );
            guard_x.push(x);
            guard_y.push(y);
        }

        let cluster_value = if dt == 1 {
            let mean_x = guard_x.iter().copied().sum::<i32>() as f64 / guard_x.len() as f64;
            guard_x.iter().map(|&x| ((x as f64 - mean_x).powi(2)) as f64).sum::<f64>().sqrt()
        } else {
            let mean_y = guard_y.iter().copied().sum::<i32>() as f64 / guard_y.len() as f64;
            guard_y.iter().map(|&y| ((y as f64 - mean_y).powi(2)) as f64).sum::<f64>().sqrt()
        };

        iterations.push_back(cluster_value);
        if iterations.len() > 10 {
            iterations.pop_front();
        }

        let mean_cluster: f64 = iterations.iter().copied().sum::<f64>() / iterations.len() as f64;

        if cluster_value < mean_cluster * 0.8 && dt == 1 {
            iterations.clear();
            dt = width;
        } else if cluster_value < mean_cluster * 0.8 && dt == width {
            return t;
        } else if t > 10000 {
            panic!("No cluster found");
        }

        t += dt;
    }
}

fn main() {
    let input_file = "input.txt";
    let width = 101;
    let height = 103;
    let seconds = 100;

    match parse_input(input_file) {
        Ok(robots) => {
            // Part 1
            let grid = simulate_motion(&robots, width, height, seconds);
            let safety_factor = calculate_safety_factor(&grid);
            println!("Safety Factor: {}", safety_factor);

            // Part 2
            let cluster_time = time_to_cluster(&robots, width, height);
            println!("Time to Cluster: {}", cluster_time);
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
}
