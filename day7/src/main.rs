use std::{fs::File, io::{self, BufRead, BufReader}};

/// Represents possible operators
#[derive(Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

// part 2
/// Concatenate two numbers
fn concatenate(a: i64, b: i64) -> i64 {
    let b_str = b.to_string();
    let concat_str = format!("{}{}", a, b_str);
    concat_str.parse().unwrap()
}

/// Solve an equation by evaluating left-to-right with given operators
fn solve_equation(numbers: &[i64], operators: &[Operator]) -> i64 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            Operator::Add => result += numbers[i + 1],
            Operator::Multiply => result *= numbers[i + 1],
            Operator::Concatenate => result = concatenate(result, numbers[i + 1]), // part 2
        }
    }
    result
}

/// Generate all possible operator combinations
fn generate_operator_combinations(num_operators: usize) -> Vec<Vec<Operator>> {
    let mut combinations = Vec::new();
    
    // Helper function to recursively generate combinations
    fn generate(current: &mut Vec<Operator>, remaining: usize, combinations: &mut Vec<Vec<Operator>>) {
        if remaining == 0 {
            combinations.push(current.clone());
            return;
        }
        
        for &op in &[Operator::Add, Operator::Multiply, Operator::Concatenate] {
            current.push(op);
            generate(current, remaining - 1, combinations);
            current.pop();
        }
    }
    
    let mut temp = Vec::new();
    generate(&mut temp, num_operators, &mut combinations);
    combinations
}

/// Check if an equation can be solved
fn is_equation_solvable(test_value: i64, numbers: &[i64]) -> bool {
    let num_operators = numbers.len() - 1;
    let operator_combinations = generate_operator_combinations(num_operators);
    
    operator_combinations.iter().any(|ops| {
        solve_equation(numbers, ops) == test_value
    })
}

fn main() -> io::Result<()> {
    let file = File::open("day7input.txt")?;
    let reader = BufReader::new(file);
    
    let mut total_calibration = 0;

    for line in reader.lines() {

        let line = line?;
        
        // Parse the line
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            continue;
        }

        let test_value: i64 = parts[0].parse().unwrap();
        let numbers: Vec<i64> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        // Check if equation is solvable
        if is_equation_solvable(test_value, &numbers) {
            total_calibration += test_value;
        }
    }

    println!("Total Calibration Result: {}", total_calibration);
    Ok(())
}
