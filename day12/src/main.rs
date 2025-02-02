use std::{fs::File, io::{self, BufRead, BufReader}};

fn main() -> io::Result<()>{
    let file = File::open("day12test.txt")?;
    let reader = BufReader::new(file);
    
    let matrix: Vec<Vec<char>> = reader.lines()
        .map(|line| line.unwrap().chars()
             .map(|x| x)
             .collect())
        .collect();
    for sub in matrix {
        println!("{:?}", sub);
    }
    Ok(())
}
