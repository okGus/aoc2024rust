use regex::Regex;
use std::{fs, io};

fn main() -> io::Result<()> {
    
    let contents: String = fs::read_to_string("day3input.txt")?;

    // Part 1
    //let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut res = 0;
    
    //for cap in re.captures_iter(&contents) {
    //    let a: i32 = cap[1].parse::<i32>().unwrap();
    //    let b: i32 = cap[2].parse::<i32>().unwrap();
    //    res += a * b;
    //}

    //println!("{}", res);

    // Part 2
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don\'t\(\)").unwrap();
    let mut compute = true;

    for cap in re.captures_iter(&contents) {
        //println!("{:?} compute = {}", cap, compute);
        if cap[0].eq("don't()") {
            compute = false;
        } else if cap[0].eq("do()") {
            compute = true;
        }
        else if compute {
            //println!("{:?}", cap);
            let a: i32 = cap[1].parse::<i32>().unwrap();
            let b: i32 = cap[2].parse::<i32>().unwrap();
            res += a * b;
        }
        //println!("{}", compute);
    }
    println!("{}", res);

    Ok(())
}
