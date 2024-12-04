use std::fs;

fn count_word(grid: &Vec<Vec<char>>, word: &str) -> usize {
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let directions = [
        (1, 0), 
        (0, 1),  
        (1, 1),  
        (-1, 0), 
        (0, -1), 
        (-1, -1), 
        (1, -1), 
        (-1, 1),
    ];

    let mut count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in directions.iter() {
                let mut match_found = true;

                for k in 0..word_len {
                    let ni = i as isize + k as isize * dx;
                    let nj = j as isize + k as isize * dy;

                    if ni < 0 || nj < 0 || ni >= rows as isize || nj >= cols as isize {
                        match_found = false;
                        break;
                    }

                    if grid[ni as usize][nj as usize] != word_chars[k] {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let word = "XMAS";

    let result = count_word(&grid, word);

    println!("The word '{}' appears {} times in the grid.", word, result);
}
