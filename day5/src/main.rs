use std::{collections::{HashMap, HashSet}, fs, io};

// Part 2 function
fn fix(arr: &mut Vec<i32>, rule: &HashMap<i32, HashSet<i32>>) -> () {
    // based on the rule_map fix the order
    // Using indexs to swap the values
    let mut idx = 0;
    let mut can_inc_idx = true;
    while idx < arr.len() {
        //println!("Rule: {:?}", rule);
        //println!("Checking: {:?}", arr);
        //println!("Current idx {}", idx);
        // if key not in map that it should be at the end
        if !rule.contains_key(&arr[idx]) && idx != arr.len()-1 {
            arr.swap(idx, idx+1);
            continue;
        }

        for jdx in idx+1..arr.len() {
            //println!("jdx: {} key: {}, value to check {}", jdx, arr[idx], arr[jdx]);
            if !rule.get(&arr[idx]).unwrap().contains(&arr[jdx]) {
                // swap the values
                arr.swap(idx, jdx);
                can_inc_idx = false;
                break;
            }
        }
        if can_inc_idx {
            idx += 1;
        } else {
            can_inc_idx = true;
        }
    }
}

fn main() -> io::Result<()> {
    let contents: String = fs::read_to_string("day5input.txt")?;
    let (top, bottom) = contents.split_once("\n\n").unwrap();

    //println!("{}", contents);
    //println!("Rules: \n{}", top);
    //println!("Page numbers: \n{}", bottom);

    let mut rule_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    
    // Go through the top line by line
    for line in top.lines() {
        // get the x and y from x|y and parse it to i32 from str
        let (xstr, ystr) = line.split_once("|").unwrap();
        let (x, y) = (xstr.parse::<i32>().unwrap(), ystr.parse::<i32>().unwrap());
        // insert key - if key exist then insert value to HashSet
        // if key doesn't exits then add new key-value pair
        rule_map.entry(x).or_insert(HashSet::new()).insert(y);
    }

    //println!("Rule Map: \n{:?}", rule_map);

    let mut skip = false;
    let mut res: Vec<i32> = vec![];
    for line in bottom.lines() {
        let mut page_numbers: Vec<i32> = line.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

        // If value in page_numbers not a key it means that it has to be infront of other numbers 
        // If value a key then the values infront [n+1:] should be in the hashset
        //println!("{:?}", page_numbers);

        for (i,v) in page_numbers.iter().enumerate() {
            // in python if i is at the end and i do [i+1:] i get an empty array
            // .skip(i+1) does the same
            let rest = page_numbers.iter().skip(i+1);
            if !rule_map.contains_key(v) && i != page_numbers.len()-1 {
                //println!("Current page_numbers: {:?} and {} not a key", page_numbers, v);
                skip = true;
                break;
            }
            for el in rest {
                if !rule_map.get(v).unwrap().contains(el) {
                    //println!("Current page_numbers: {:?} and {} not a value of key {}", page_numbers, el, v);
                    skip = true;
                    break;
                }
            }
            if skip {
                break;
            }
        }
        if skip {
            skip = false;
            // fix incorrectly-order page 
            //println!("Fixing order of {:?}", page_numbers);
            fix(&mut page_numbers, &rule_map); // part of part 2
            //println!("After fixing: {:?}", page_numbers);
            //continue; // part of part 1
            
            // part 2
            let mid = page_numbers.len() / 2;
            res.push(page_numbers[mid]);
        }
        // find mid index and add it to res
        //println!("Current page_numbers: {:?}", page_numbers);

        // the following two lines are part 1
        //let mid = page_numbers.len() / 2;
        //res.push(page_numbers[mid]);
        
        //println!("Current res: {:?}", res);
    }

    //println!("res: {:?}", res);
    let s: i32 = res.iter().sum::<i32>();
    println!("{}", s);

    Ok(())
}
