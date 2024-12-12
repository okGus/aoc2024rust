use std::{fs::File, io::{BufRead, BufReader}};

fn is_increasing(arr: &[i32]) -> bool {
    arr.windows(2).all(|a| a[0] < a[1])
}

fn is_decreasing(arr: &[i32]) -> bool {
    arr.windows(2).all(|a| a[0] > a[1])
}

fn is_bounded(arr: &[i32]) -> bool {
    for idx in 1..arr.len() {
        if (arr[idx] - arr[idx - 1]).abs() > 3 {
            return false;
        }
    }
    true
}

fn tolerate(arr: &[i32]) -> bool {
    if is_increasing(&arr) && is_bounded(&arr) {
        //println!("{:?} is increasing and within bounds", arr);
        return true
    }
    if is_decreasing(&arr) && is_bounded(&arr) {
        //println!("{:?} is decreasing and within bounds", arr);
        return true
    }

    for idx in 0..arr.len() {
        let new_arr: Vec<i32> = arr.iter()
                         .enumerate()
                         .filter(|&(i, _)| i != idx)
                         .map(|(_, &x)| x)
                         .collect();
        if is_increasing(&new_arr) && is_bounded(&new_arr) {
            //println!("{:?} is increasing and within bounds by removing index {}", new_arr, idx);
            return true;
        }
        if is_decreasing(&new_arr) && is_bounded(&new_arr) {
            //println!("{:?} is decreasing and within bounds by removing index {}", new_arr, idx);
            return true;
        }
    }
    false
}

fn main() -> Result<(), std::io::Error> {

    let file = File::open("./src/day2input.txt")?;
    let reader = BufReader::new(file);
    
    let mut arr: Vec<Vec<i32>> = vec![];
    for line in reader.lines() {
        let subarr: Vec<i32> = line?.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
        arr.push(subarr);
    }
    
    let mut safes = 0;
    //for subarr in arr {
    //    if is_increasing(&subarr) && is_bounded(&subarr) {
    //        //println!("{:?}, is increasing and levels are at most 1, 2, or 3", subarr);
    //        safes += 1;
    //    }
    //    else if is_decreasing(&subarr) && is_bounded(&subarr) {
    //        //println!("{:?}, is decreasing and levels are at most 1, 2, or 3", subarr);
    //        safes += 1;
    //    }
    //    //else {
    //    //    println!("{:?}, array is increasing/increasing but levels are more than 3", subarr);
    //    //}
    //}


    // Part 2
    for subarr in arr {
        //println!("Checking {:?}", subarr);
        let is_safe = tolerate(&subarr);
        //println!("{:?} Safe: {}", subarr, is_safe);
        if is_safe {
            safes += 1;
        }
    }

    println!("{}", safes);

    Ok(())
}
