use std::{collections::HashSet, fs::File, io::{self, BufRead, BufReader}};

fn is_bounded(matrix: &Vec<Vec<i32>>, r: i32, c: i32) -> bool {
    if r >= 0 && c >= 0 && r < matrix.len() as i32 && c < matrix[0].len() as i32 {
        return true;
    }
    false
}

fn dfs(matrix: &Vec<Vec<i32>>, 
       seen: &mut HashSet<(i32, i32)>,
       r: i32, 
       c: i32,
       res: &mut i32) -> () {
    // bounds checking
    if !is_bounded(matrix, r, c) || seen.contains(&(r, c)) {
        return;
    }
    let val: i32 = matrix[r as usize][c as usize];
    //seen.insert((r, c)); // part 1 needed, part 2 doesn't

    if val == 9 {
        *res += 1;
        return;
    }

    if r - 1 >= 0 && val + 1 == matrix[(r - 1) as usize][c as usize] {
        dfs(matrix, seen, r - 1, c, res); // up
    }
    if c + 1 < matrix[0].len() as i32 && val + 1 == matrix[r as usize][(c + 1) as usize] {
        dfs(matrix, seen, r, c + 1, res); // right
    }
    if r + 1 < matrix.len() as i32 && val + 1 == matrix[(r + 1) as usize][c as usize] {
        dfs(matrix, seen, r + 1, c, res); // down 
    }
    if c - 1 >= 0 && val + 1 == matrix[r as usize][(c - 1) as usize] {
        dfs(matrix, seen, r, c - 1, res); // left
    }
}

fn solution(matrix: &mut Vec<Vec<i32>>) -> () {
    let mut total_score = 0;
    let mut trailhead_scores: Vec<i32> = Vec::new();

    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            if matrix[r][c] == 0 {
                let mut seen: HashSet<(i32, i32)> = HashSet::new();
                let mut trailhead_score = 0;
                dfs(matrix, &mut seen, r as i32, c as i32, &mut trailhead_score);

                trailhead_scores.push(trailhead_score);
                total_score += trailhead_score;
            }
        }
    }
    //println!("{}", res);
    println!("Trailhead Scores: {:?}", trailhead_scores);
    println!("Total Trailhead Score: {}", total_score);
}

fn main() -> io::Result<()> {
    let file = File::open("day10input.txt")?;
    let reader = BufReader::new(file);
    
    let matrix: Vec<Vec<i32>> = reader.lines()
        .map(|line| line.unwrap().chars()
             .map(|x| x.to_digit(10).unwrap() as i32)
             .collect())
        .collect();

    for sub in &matrix {
        println!("{:?}", sub);
    }

    solution(&mut matrix.clone());

    Ok(())
}

