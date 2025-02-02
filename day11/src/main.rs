use std::collections::HashMap;

// Define the recursive function with caching
fn ans(x: u64, n: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    // Base case: if n == 0, return 1
    println!("(x, n): ({}, {}) cache: {:?}", x, n, cache);
    if n == 0 {
        return 1;
    }

    // Check if the result is already in the cache
    if let Some(&cached_result) = cache.get(&(x, n)) {
        return cached_result;
    }

    // Compute the result if not cached
    let result = if x == 0 {
        ans(1, n - 1, cache) // Replace 0 with 1 and reduce n
    } else if x.to_string().len() % 2 == 0 {
        // Split the number if it has an even number of digits
        let x_str = x.to_string();
        let mid = x_str.len() / 2;
        let first_half: u64 = x_str[..mid].parse().unwrap();
        let second_half: u64 = x_str[mid..].parse().unwrap();
        ans(first_half, n - 1, cache) + ans(second_half, n - 1, cache)
    } else {
        // Multiply the number by 2024 if it has an odd number of digits
        ans(2024 * x, n - 1, cache)
    };

    // Cache the result
    cache.insert((x, n), result);

    // Return the result
    result
}

fn main() {
    use std::fs;

    // Read input numbers from file
    let input = fs::read_to_string("day11input.txt").expect("Failed to read input file");
    let nums: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    // Initialize the cache
    let mut cache: HashMap<(u64, usize), usize> = HashMap::new();
    let steps = 75;

    // Compute the result
    let mut total_result = 0;
    //for &x in &nums {
    //    total_result += ans(x, steps, &mut cache);
    //}
    let f = ans(2024, 3, &mut cache);
    println!("Total result: {} {:?}", f, cache);
    // Print the final result
    println!("Total result: {}", total_result);
}

