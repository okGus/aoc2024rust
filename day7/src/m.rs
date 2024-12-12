use std::{fs::File, io::{self, BufRead, BufReader}};
use regex::Regex;

// In rust i have to do 'a - an ugly way to resolve lifetime
fn create_op_str<'a>(v: &mut Vec<&'a str>, 
                    plus: &'a Vec<&'a str>, 
                    rest: &'a Vec<&'a str>) -> () {
    let mut idx = 0;
    while idx < plus.len() {
        v.push(rest[idx]);
        v.push(plus[idx]);
        idx += 1;
    }
    v.push(rest[idx]);
}

fn equals(answer: &str, arr_str: &Vec<&str>) -> bool {
    let mut stack: Vec<String> = vec![];
    
    for c in arr_str {
        if *c == "+" || *c == "*" {
            stack.push(c.to_string());
        }
        else if !stack.is_empty() {
            let operator = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            let res: i64;

            if operator == "+" {
                res = a.parse::<i64>().unwrap() + c.parse::<i64>().unwrap();
            } else if operator == "*" {
                res = a.parse::<i64>().unwrap() * c.parse::<i64>().unwrap();
            } else {
                unreachable!("Either + or * must be true"); // rust compiler - i'm the guarantee not you
            }
            stack.push(res.to_string());
        } else {
            stack.push(c.to_string());
        }
        //println!("####### {:?}", stack);
    }
    //println!("Ending {:?}", stack);
    if stack[0].parse::<i64>().unwrap() == answer.parse::<i64>().unwrap() {
        return true;
    }
    false
}

fn possible_operations(answer: &str, rest: &mut Vec<&str>) -> i64 {
    let plus: Vec<&str> = vec!["+"; rest.len()-1];
    //let mut start = 0;
    let mut end = 0;
    //println!("Array: {:?}", rest);
    //println!("Plus: {:?}", plus);

    let mut v: Vec<&str> = vec![];
    create_op_str(&mut v, &plus, rest);
    
    if equals(answer, &v) {
        return answer.parse::<i64>().unwrap();
    }

    while end < plus.len() {

        //println!("end: {}", end);
        let mut s = 0;
        let mut e = end;

        //println!("e: {}", e);
        while e < plus.len() {
            let mut plus_c: Vec<&str> = plus.clone();
            // add multipliers
            for i in s..e+1 {
                plus_c[i] = "*";   
            }
            //println!("s: {} e: {} arr: {:?}", s, e, plus_c);
            s += 1;
            e += 1;

            // Create a stack of numbers and operators
            v = Vec::new();
            create_op_str(&mut v, &plus_c, rest);
            if equals(answer, &v) {
                return answer.parse::<i64>().unwrap();
            }

            //println!("V: {:?}", v);
        }
        end += 1;
    }

    0
}

fn main() -> io::Result<()> {
    // ['3', '', '2', '', '6', '', '4']
    //
    // [3, 2, 6, 4]
    // lenght = 4 elements
    // ['+', '+', '+']
    //
    // start idx = 0
    // end idx = 1
    //
    // replace from [0:1] with '*'
    // ['*', '+', '+']
    //
    // start idx = 1
    // end idx = 2
    // replace from [1:2] with '*' and so on 
    //
    // then having this
    // ['+', '+', '+']
    // start idx = 0
    // end idx = 2 so one more
    // so replace from [0:2] with '*'
    let file = File::open("day7input.txt")?;
    let reader = BufReader::new(file);
    
    let re = Regex::new(r"(\d+)").unwrap();
    let mut res: i64 = 0;

    for line in reader.lines() {
        let l = line?;
        let numbers: Vec<&str> = re.find_iter(l.as_str()).map(|x| x.as_str()).collect();
        let answer: &str = numbers[0];
        let mut rest: Vec<&str> = numbers[1..].to_vec();
        
        res += possible_operations(answer, &mut rest);
    }
    println!("{}", res);

    Ok(())
}
