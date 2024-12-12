use itertools::Itertools;
use std::{fs::File, io::{self, BufRead, BufReader}};

fn main() -> io::Result<()> {
    let file = File::open("day4input.txt")?;
    let reader = BufReader::new(file);
    
    let mut arr: Vec<Vec<char>> = vec![];
    let mut res = 0;

    for line in reader.lines() {
        let subarr: Vec<char> = line?.chars().collect();
        arr.push(subarr);
    }

    //for (_, subarr) in arr.iter().enumerate() {
    //    println!("{:?}", subarr);
    //}

    // Part 1
    let rows = arr.len();
    let cols = arr[0].len();
    
    //for row in 0..rows {
    //    for col in 0..cols {
    //        // check up 
    //        if row >= 3 && arr[row][col] == 'X' && arr[row-1][col] == 'M' && arr[row-2][col] == 'A' && arr[row-3][col] == 'S' {
    //            res += 1;
    //        }
    //        // check diagonal up right
    //        if row >= 3 && col + 3 < cols && arr[row][col] == 'X' && arr[row-1][col+1] == 'M' && arr[row-2][col+2] == 'A' && arr[row-3][col+3] == 'S' {
    //            res += 1;
    //        }
    //        // check right
    //        if col + 3 < cols && arr[row][col] == 'X' && arr[row][col+1] == 'M' && arr[row][col+2] == 'A' && arr[row][col+3] == 'S' {
    //            res += 1;
    //        }
    //        // check diagonal bottom right 
    //        if row + 3 < rows && col + 3 < cols && arr[row][col] == 'X' && arr[row+1][col+1] == 'M' && arr[row+2][col+2] == 'A' && arr[row+3][col+3] == 'S' {
    //            res += 1;
    //        }
    //        // check bottom 
    //        if row + 3 < rows && arr[row][col] == 'X' && arr[row+1][col] == 'M' && arr[row+2][col] == 'A' && arr[row+3][col] == 'S' {
    //            res += 1;
    //        }
    //        // check diagonal bottom left 
    //        if row + 3 < rows && col >= 3 && arr[row][col] == 'X' && arr[row+1][col-1] == 'M' && arr[row+2][col-2] == 'A' && arr[row+3][col-3] == 'S' {
    //            res += 1;
    //        }
    //        // check left 
    //        if col >= 3 && arr[row][col] == 'X' && arr[row][col-1] == 'M' && arr[row][col-2] == 'A' && arr[row][col-3] == 'S' {
    //            res += 1;
    //        }
    //        // check diagonal up left 
    //        if row >= 3 && col >= 3 && arr[row][col] == 'X' && arr[row-1][col-1] == 'M' && arr[row-2][col-2] == 'A' && arr[row-3][col-3] == 'S' {
    //            res += 1;
    //        }
    //    }
    //}

    println!("{}", res);

    // Part 2 - use the 'A' to determine if it's a cross
    for row in 0..rows {
        for col in 0..cols {
            if arr[row][col] == 'A' {
                // I can go up in row, i can go left in col, i can go down in row, i can go right in col
                if row >= 1 && col >= 1 && row + 1 < rows && col + 1 < cols {
                    let one: String = String::from(arr[row][col].to_string() + &arr[row-1][col-1].to_string() + &arr[row+1][col+1].to_string());
                    let one_chars: String = one.chars().sorted().collect();

                    let two = String::from(arr[row][col].to_string() + &arr[row-1][col+1].to_string() + &arr[row+1][col-1].to_string());
                    let two_chars: String = two.chars().sorted().collect();
                     
                    if one_chars == "AMS" && two_chars == "AMS" {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("{}", res);

    Ok(())
}
