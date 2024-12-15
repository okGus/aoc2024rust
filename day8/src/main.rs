use std::{fs::File, io::{self, BufRead, BufReader}};

fn print_matrix(matrix: &Vec<Vec<char>>) -> () {
    println!("Matrix: ");
    for subarr in matrix {
        println!("{:?}", subarr);
    }
}

fn create_antinode(antinodes: &mut Vec<Vec<char>>, sr: i32, sc: i32, i: i32, j: i32) {

    let diff_x: i32 = i - sr;
    let diff_y: i32 = j - sc; 
    
    let n_x: i32 = diff_x + i;
    let n_y: i32 = diff_y + j;

    let rows: i32 = antinodes.len() as i32;
    let cols: i32 = antinodes[0].len() as i32;

    // check bounds
    if n_x < 0 || n_y < 0 || n_x >= rows || n_y >= cols {
        return;
    }

    antinodes[n_x as usize][n_y as usize] = '#';
    create_antinode(antinodes, i, j, n_x, n_y); // recuring is part 2, rest is all part 1 
}

fn solve(antennas: &Vec<Vec<char>>, antinodes: &mut Vec<Vec<char>>) {
    let mut ants: Vec<(char, i32, i32)> = vec![];
    // find all characters and their location (x,y)
    for (i, ant) in antennas.iter().enumerate() {
        for (j, &c) in ant.iter().enumerate() {
            if antennas[i][j] != '.' {
                ants.push((c, i as i32, j as i32));
            }
        }
    }

    // loop through this new array, and create antinodes
    for (i, &(c, sr, sc)) in ants.iter().enumerate() {
        for (j, &(ch, x, y)) in ants.iter().enumerate() {
            if i == j {
                continue;
            }
            if c == ch {
                antinodes[sr as usize][sc as usize] = '#'; // part 2 only
                antinodes[x as usize][y as usize] = '#'; // part 2 only
                create_antinode(antinodes, sr, sc, x, y);
            }

        } 
    }
}

fn main() -> io::Result<()> {
    let file = File::open("day8input.txt")?;
    let reader = BufReader::new(file);
    
    let mut antennas: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let subarr: Vec<char> = line?.chars().collect();
        antennas.push(subarr);
    }

    //print_matrix(&antennas);
    let mut antinodes: Vec<Vec<char>> = vec![vec!['.'; antennas[0].len()]; antennas.len()];

    solve(&antennas, &mut antinodes);
    //print_matrix(&antinodes);
    
    let mut res = 0;
    for r in 0..antinodes.len() {
        for c in 0..antinodes[0].len() {
            if antinodes[r][c] == '#' {
                res += 1;
            }
        }
    }

    println!("{}", res);

    Ok(())
}
