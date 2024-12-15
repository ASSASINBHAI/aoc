use std::{collections::VecDeque, fs};

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Box,
    BoxRight,
}

fn part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let mut width = 0;
    let mut pos = 0;

    let mut map: Vec<_> = lines.by_ref().take_while(|line| !line.is_empty()).enumerate().flat_map(|(y, line)| {
        width = line.len();
        if let Some(x) = line.chars().position(|c| c == '@') {
            pos = x as isize + y as isize * width as isize;
        }

        line.chars().map(|c| {
            match c {
                '.' | '@' => Tile::Empty,
                '#' => Tile::Wall,
                'O' => Tile::Box,
                _ => panic!("Invalid tile"),
            }
        })
    }).collect();

    for line in lines {
        for command in line.chars() {
            let direction = match command {
                '^' => -(width as isize),
                'v' => width as isize,
                '<' => -1,
                '>' => 1,
                _ => panic!("Invalid command"),
            };

            let mut test_pos = pos;
            loop {
                test_pos += direction;
                match map[test_pos as usize] {
                    Tile::Empty => {
                        pos += direction;
                        map[test_pos as usize] = map[pos as usize];
                        map[pos as usize] = Tile::Empty;
                        break;
                    }
                    Tile::Wall => break,
                    Tile::Box => {}
                    Tile::BoxRight => panic!(),
                }
            }
        }
    }

    map.into_iter().enumerate().filter_map(|(pos, tile)| {
        match tile {
            Tile::Box => Some(pos % width + pos / width * 100),
            _ => None,
        }
    }).sum()
}

fn part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let mut width = 0;
    let mut pos = 0;

    let mut map: Vec<_> = lines.by_ref().take_while(|line| !line.is_empty()).enumerate().flat_map(|(y, line)| {
        width = line.len() * 2;
        if let Some(x) = line.chars().position(|c| c == '@') {
            pos = x as isize * 2 + y as isize * width as isize;
        }

        line.chars().flat_map(|c| {
            match c {
                '.' | '@' => [Tile::Empty; 2],
                '#' => [Tile::Wall; 2],
                'O' => [Tile::Box, Tile::BoxRight],
                _ => panic!("Invalid tile"),
            }
        })
    }).collect();

    let mut boxes_to_move = Vec::new();
    let mut positions_to_check = VecDeque::new();

    for line in lines {
        for command in line.chars() {
            let direction = match command {
                '^' => -(width as isize),
                'v' => width as isize,
                '<' => -1,
                '>' => 1,
                _ => panic!("Invalid command"),
            };

            let mut blocked = false;
            positions_to_check.push_back(pos + direction);

            while let Some(position_to_check) = positions_to_check.pop_front() {
                if let Some(box_to_move) = match map[position_to_check as usize] {
                    Tile::Empty => None,
                    Tile::Wall => {
                        blocked = true;
                        break;
                    }
                    Tile::Box => Some(position_to_check),
                    Tile::BoxRight => Some(position_to_check - 1),
                } {
                    if !boxes_to_move.contains(&box_to_move) {
                        boxes_to_move.push(box_to_move);
                        if direction != 1 {
                            positions_to_check.push_back(box_to_move + direction);
                        }
                        if direction != -1 {
                            positions_to_check.push_back(box_to_move + 1 + direction);
                        }
                    }
                }
            }

            if !blocked {
                pos += direction;
                for &box_to_move in boxes_to_move.iter().rev() {
                    map[box_to_move as usize] = Tile::Empty;
                    map[box_to_move as usize + 1] = Tile::Empty;
                    map[(box_to_move + direction) as usize] = Tile::Box;
                    map[(box_to_move + 1 + direction) as usize] = Tile::BoxRight;
                }
            }

            boxes_to_move.clear();
            positions_to_check.clear();
        }
    }

    map.into_iter().enumerate().filter_map(|(pos, tile)| {
        match tile {
            Tile::Box => Some(pos % width + pos / width * 100),
            _ => None,
        }
    }).sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");

    let start_time = std::time::Instant::now();
    let result = part_1(&input);
    println!("Part 1 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 1 result: {}", result);

    let start_time = std::time::Instant::now();
    let result = part_2(&input);
    println!("Part 2 time: {:?}", std::time::Instant::now() - start_time);
    println!("Part 2 result: {}", result);
}
