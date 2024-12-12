use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader}};

fn main() -> io::Result<()> {
    let file = File::open("./src/day1input.txt")?;
    let reader = BufReader::new(file);

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in reader.lines() {
        let line_str = line?;
        let input: Vec<&str> = line_str.split(" ").collect();
        left.push(input[0].parse().unwrap());
        right.push(input[3].parse().unwrap());
    }

    left.sort();
    right.sort();

    let mut total = 0;
    for idx in 0..left.len() {
        let diff = (left[idx] - right[idx]).abs();
        total += diff;
    }

    println!("{}", total);

    // Part Two
    let mut right_counts = HashMap::new();

    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }
    
    let mut total = 0;
    for &num in &left {
        let occ = right_counts.get(&num).unwrap_or(&0);
        total += num * occ;
    }

    println!("{}", total);

    Ok(())
}
