use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RegistryType {
    A,
    B,
    C,
}

#[derive(Debug, Clone)]
struct Registry {
    value: i64,
}

impl Registry {
    fn new(value: i64) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    ADV = 0, 
    BXL = 1, 
    BST = 2, 
    JNZ = 3, 
    BXC = 4, 
    OUT = 5, 
    BDV = 6, 
    CDV = 7, 
}

impl Opcode {
    fn from(value: i64) -> Option<Self> {
        match value {
            0 => Some(Opcode::ADV),
            1 => Some(Opcode::BXL),
            2 => Some(Opcode::BST),
            3 => Some(Opcode::JNZ),
            4 => Some(Opcode::BXC),
            5 => Some(Opcode::OUT),
            6 => Some(Opcode::BDV),
            7 => Some(Opcode::CDV),
            _ => None,
        }
    }
}

fn parse_input(file_name: &str) -> (HashMap<RegistryType, Registry>, Vec<i64>) {
    let content = fs::read_to_string(file_name).expect("Failed to read input file");
    let lines: Vec<&str> = content.lines().collect();

    if lines.len() < 5 {
        panic!("Input file does not have enough lines. Expected at least 5 lines.");
    }

    let mut registries = HashMap::new();

    // Parse registry values
    for (i, &line) in lines.iter().take(3).enumerate() {
        if let Some((_, value)) = line.split_once(":") {
            let value = value.trim().parse::<i64>().expect(&format!(
                "Invalid registry value on line {}: '{}'",
                i + 1,
                line
            ));
            let registry_type = match i {
                0 => RegistryType::A,
                1 => RegistryType::B,
                2 => RegistryType::C,
                _ => unreachable!(),
            };
            registries.insert(registry_type, Registry::new(value));
        } else {
            panic!("Invalid registry format on line {}: '{}'", i + 1, line);
        }
    }

    // Parse program instructions
    let program = if let Some((_, values)) = lines[4].split_once(":") {
        values
            .trim()
            .split(',')
            .map(|x| x.parse::<i64>().expect(&format!("Invalid program value: '{}'", x)))
            .collect()
    } else {
        panic!("Invalid program format on line 5: '{}'", lines[4]);
    };

    println!("Parsed registries: {:?}", registries);
    println!("Parsed program: {:?}", program);

    (registries, program)
}

fn combo(operand: i64, registries: &HashMap<RegistryType, Registry>) -> i64 {
    match operand {
        0..=3 => operand,
        4 => registries[&RegistryType::A].value,
        5 => registries[&RegistryType::B].value,
        6 => registries[&RegistryType::C].value,
        _ => panic!("Invalid operand"),
    }
}

fn evaluate(
    registries: &mut HashMap<RegistryType, Registry>,
    program: &[i64],
) -> Vec<i64> {
    let mut output = Vec::new();
    let mut pointer = 0;

    while pointer < program.len() {
        let opcode = Opcode::from(program[pointer]).unwrap();
        let operand = program[pointer + 1];

        match opcode {
            Opcode::ADV | Opcode::BDV | Opcode::CDV => {
                let target = match opcode {
                    Opcode::ADV => RegistryType::A,
                    Opcode::BDV => RegistryType::B,
                    Opcode::CDV => RegistryType::C,
                    _ => unreachable!(),
                };
                let shift = combo(operand, registries);
                registries.get_mut(&target).unwrap().value =
                    registries[&RegistryType::A].value >> shift;
            }
            Opcode::BXL => {
                registries.get_mut(&RegistryType::B).unwrap().value ^=
                    operand;
            }
            Opcode::BST => {
                registries.get_mut(&RegistryType::B).unwrap().value =
                    combo(operand, registries) % 8;
            }
            Opcode::JNZ => {
                if registries[&RegistryType::A].value != 0 {
                    pointer = operand as usize;
                    continue;
                }
            }
            Opcode::BXC => {
                registries.get_mut(&RegistryType::B).unwrap().value ^=
                    registries[&RegistryType::C].value;
            }
            Opcode::OUT => {
                output.push(combo(operand, registries) % 8);
            }
        }

        pointer += 2;
    }

    output
}

fn part_1(file_name: &str) -> String {
    let (mut registries, program) = parse_input(file_name);
    let result = evaluate(&mut registries, &program);
    result.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")
}

fn part_2(file_name: &str) -> i64 {
    let (mut registries, program) = parse_input(file_name);
    let mut candidates = vec![0];

    for length in 1..=program.len() {
        let mut new_candidates = Vec::new();

        for num in &candidates {
            for offset in 0..8 {
                let a = 8 * num + offset;
                registries.insert(RegistryType::A, Registry::new(a));
                if evaluate(&mut registries.clone(), &program) == program[program.len() - length..] {
                    new_candidates.push(a);
                }
            }
        }

        candidates = new_candidates;
        if candidates.is_empty() {
            break;
        }
    }

    *candidates.iter().min().unwrap_or(&-1)
}

fn main() {
    let input_file = "input.txt";

    // Part 1 Solution
    println!("Part 1 Solution: {}", part_1(input_file));

    // Part 2 Solution
    println!("Part 2 Solution: {}", part_2(input_file));
}
