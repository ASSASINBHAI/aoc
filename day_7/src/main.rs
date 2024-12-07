use std::fs;

struct OperatorCalibrationSolver;

impl OperatorCalibrationSolver {
    fn evaluate_expression(nums: &[i64], ops: &[char]) -> i64 {
        let mut result = nums[0];
        for (i, op) in ops.iter().enumerate() {
            match op {
                '+' => result += nums[i + 1],
                '*' => result *= nums[i + 1],
                '|' => {
                    let concat = format!("{}{}", result, nums[i + 1]);
                    result = concat.parse::<i64>().unwrap_or_default();
                }
                _ => panic!("Invalid operator"),
            }
        }
        result
    }

    fn generate_possible_results(
        test_value: i64,
        numbers: &[i64],
        allow_concatenation: bool,
    ) -> Vec<i64> {
        let operator_options = if allow_concatenation { vec!['+', '*', '|'] } else { vec!['+', '*'] };
        let op_slots = numbers.len() - 1;

        let mut results = vec![];

        for ops_combo in (0..operator_options.len().pow(op_slots as u32)).map(|mut x| {
            (0..op_slots)
                .map(|_| {
                    let op = operator_options[x % operator_options.len()];
                    x /= operator_options.len();
                    op
                })
                .collect::<Vec<char>>()
        }) {
            if let Ok(result) = std::panic::catch_unwind(|| Self::evaluate_expression(numbers, &ops_combo)) {
                if result == test_value {
                    results.push(result);
                }
            }
        }

        results
    }

    fn solve_calibration(
        equations: Vec<(i64, Vec<i64>)>,
        allow_concatenation: bool,
    ) -> i64 {
        equations
            .iter()
            .filter_map(|(test_value, numbers)| {
                if !Self::generate_possible_results(*test_value, numbers, allow_concatenation).is_empty() {
                    Some(*test_value)
                } else {
                    None
                }
            })
            .sum()
    }

    fn parse_input(filename: &str) -> Vec<(i64, Vec<i64>)> {
        fs::read_to_string(filename)
            .expect("Failed to read file")
            .lines()
            .filter_map(|line| {
                let mut parts = line.split(':');
                let test_value = parts.next()?.trim().parse().ok()?;
                let numbers = parts.next()?
                    .split_whitespace()
                    .filter_map(|n| n.parse().ok())
                    .collect();
                Some((test_value, numbers))
            })
            .collect()
    }
}

fn main() {
    let input_file = "input.txt";

    let equations = OperatorCalibrationSolver::parse_input(input_file);

    let part_one_result = OperatorCalibrationSolver::solve_calibration(equations.clone(), false);
    println!("Part 1 Total Calibration Result: {}", part_one_result);

    let part_two_result = OperatorCalibrationSolver::solve_calibration(equations, true);
    println!("Part 2 Total Calibration Result: {}", part_two_result);
}
