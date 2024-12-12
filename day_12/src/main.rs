use std::collections::VecDeque;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point2D {
    x: isize,
    y: isize,
}

impl Point2D {
    fn new(x: isize, y: isize) -> Self {
        Point2D { x, y }
    }

    fn adjacent_points(&self) -> Vec<Point2D> {
        vec![
            Point2D::new(self.x - 1, self.y),
            Point2D::new(self.x + 1, self.y),
            Point2D::new(self.x, self.y - 1),
            Point2D::new(self.x, self.y + 1),
        ]
    }
}

impl std::ops::Add for Point2D {
    type Output = Point2D;

    fn add(self, other: Point2D) -> Point2D {
        Point2D::new(self.x + other.x, self.y + other.y)
    }
}

struct Grid2D {
    data: Vec<Vec<u8>>,
}

impl Grid2D {
    fn from_input_lines(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| line.bytes().collect())
            .collect();
        Grid2D { data }
    }

    fn row_count(&self) -> usize {
        self.data.len()
    }

    fn col_count(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        }
    }

    fn contains(&self, point: Point2D) -> bool {
        point.y >= 0
            && point.y < self.row_count() as isize
            && point.x >= 0
            && point.x < self.col_count() as isize
    }
}

impl std::ops::Index<Point2D> for Grid2D {
    type Output = u8;

    fn index(&self, point: Point2D) -> &Self::Output {
        &self.data[point.y as usize][point.x as usize]
    }
}

fn get_number_of_corners(point: Point2D, grid: &Grid2D) -> u64 {
    let mut number_of_corners = 0;

    let mut matches = [[false; 3]; 3];

    let value = grid[point];
    for r_d in 0..3 {
        for c_d in 0..3 {
            let compare_point = point + Point2D::new(c_d as isize - 1, r_d as isize - 1);
            matches[r_d][c_d] = grid.contains(compare_point) && grid[compare_point] == value;
        }
    }

    if matches[0][1] {
        if matches[1][0] && !matches[0][0] {
            number_of_corners += 1;
        }

        if matches[1][2] && !matches[0][2] {
            number_of_corners += 1;
        }
    } else {
        if !matches[1][0] {
            number_of_corners += 1;
        }

        if !matches[1][2] {
            number_of_corners += 1;
        }
    }

    if matches[2][1] {
        if matches[1][2] && !matches[2][2] {
            number_of_corners += 1;
        }

        if matches[1][0] && !matches[2][0] {
            number_of_corners += 1;
        }
    } else {
        if !matches[1][0] {
            number_of_corners += 1;
        }

        if !matches[1][2] {
            number_of_corners += 1;
        }
    }

    number_of_corners
}

fn solve<const COUNT_PERIMETER: bool>(input: &str) -> u64 {
    let grid = Grid2D::from_input_lines(input);
    let mut visited = vec![vec![false; grid.col_count()]; grid.row_count()];
    let mut queue = VecDeque::with_capacity(32);

    let mut cost = 0;
    for r in 0..grid.row_count() {
        for c in 0..grid.col_count() {
            let starting_point = Point2D::new(c as isize, r as isize);
            if visited[r][c] {
                continue;
            }

            let starting_value = grid[starting_point];

            let mut area = 0;
            let mut edge_cost = 0;
            queue.push_back(starting_point);
            visited[r][c] = true;
            while let Some(current_point) = queue.pop_front() {
                area += 1;
                if !COUNT_PERIMETER {
                    edge_cost += get_number_of_corners(current_point, &grid);
                }

                for adjacent_point in current_point.adjacent_points() {
                    if !grid.contains(adjacent_point) {
                        if COUNT_PERIMETER {
                            edge_cost += 1;
                        }
                    } else {
                        let ar = adjacent_point.y as usize;
                        let ac = adjacent_point.x as usize;
                        if grid[adjacent_point] == starting_value {
                            if !visited[ar][ac] {
                                visited[ar][ac] = true;
                                queue.push_back(adjacent_point);
                            }
                        } else if COUNT_PERIMETER {
                            edge_cost += 1;
                        }
                    }
                }
            }

            cost += area * edge_cost;
        }
    }

    cost
}

pub fn part1(input: &str) -> u64 {
    solve::<true>(input)
}

pub fn part2(input: &str) -> u64 {
    solve::<false>(input)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    let result_part1 = part1(&input);
    println!("Part 1: {}", result_part1);

    let result_part2 = part2(&input);
    println!("Part 2: {}", result_part2);
}