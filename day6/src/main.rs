use std::{collections::{HashMap, HashSet}, fs::File, io::{self, BufRead, BufReader}};

fn find_initial_position(arr: &Vec<Vec<char>>) -> (usize, usize) {
    for row in 0..arr.len() {
        for col in 0..arr[0].len() {
            if arr[row][col] == '^' {
                return (row, col);
            }
        }
    }
    unreachable!("The '^' character was not found in the matrix.");
}

fn print_matrix(matrix: &Vec<Vec<char>>) -> () {
    println!("Matrix: ");
    for subarr in matrix {
        println!("{:?}", subarr);
    }
}

// part 1
fn start_moving(matrix: &mut Vec<Vec<char>>, 
                mm: &mut Vec<Vec<char>>, 
                x: &mut usize, 
                y: &mut usize,
                rows: usize,
                cols: usize) -> () {
    // Start moving '^' till it goes out of range
    // Define some dummy conditions for movement (replace with actual logic)
    let mut can_move_up = true;
    let mut can_move_down = false;
    let mut can_move_left = false;
    let mut can_move_right = false;

    // usize cannot represent negative numbers, 
    // you don't need to include conditions like x >= 0 or y >= 0 in your loop condition.
    while *x < rows && *y < cols {
        if can_move_up {
            if *x - 1 < 0 {
                break;
            }
            if matrix[*x - 1][*y] != '#' {
                matrix[*x][*y] = '.';
                *x -= 1;
                matrix[*x][*y] = '^';
                mm[*x][*y] = '#';
            } else {
                can_move_up = false;
                can_move_right = true;
                matrix[*x][*y] = '>';
            }
        }
        if can_move_right {
            if *y + 1 >= cols {
                break;
            }
            if matrix[*x][*y + 1] != '#' {
                matrix[*x][*y] = '.';
                *y += 1;
                matrix[*x][*y] = '>';
                mm[*x][*y] = '#';
            } else {
                can_move_right = false;
                can_move_down = true;
                matrix[*x][*y] = 'v';
            }
        }
        if can_move_down {
            if *x + 1 >= rows {
                break;
            }
            if matrix[*x + 1][*y] != '#' {
                matrix[*x][*y] = '.';
                *x += 1;
                matrix[*x][*y] = 'v';
                mm[*x][*y] = '#';
            } else {
                can_move_down = false;
                can_move_left = true;
                matrix[*x][*y] = '<';
            }
        }
        if can_move_left {
            if *y - 1 < 0 {
                break;
            }
            if matrix[*x][*y - 1] != '#'  {
                matrix[*x][*y] = '.';
                *y -= 1;
                matrix[*x][*y] = '<';
                mm[*x][*y] = '#';
            } else {
                can_move_left = false;
                can_move_up = true;
                matrix[*x][*y] = '^';
            }
        }

        //print_matrix(&matrix);
        //print_matrix(&mm);
    }

    // find distinct positions 
    let mut res = 0;
    for subarr in mm {
        for char in subarr {
            if *char == '#'  {
                res += 1;
            }
        }
    }
    println!("Result: {}", res);
}

// Part 2
fn has_loop(cloned_matrix: &mut Vec<Vec<char>>,
            pos_map: &mut HashMap<(usize, usize), char>,
            x_: &mut usize,
            y_: &mut usize) -> bool {

    let mut can_move_up = true;
    let mut can_move_down = false;
    let mut can_move_left = false;
    let mut can_move_right = false;
    let rows = cloned_matrix.len();
    let cols = cloned_matrix[0].len();
    let mut x = *x_;
    let mut y = *y_;

    while x < rows && y < cols {
        //print_matrix(&cloned_matrix);

        if can_move_up {
            if x == 0 {
                break;
            }
            if cloned_matrix[x - 1][y] == '#' || cloned_matrix[x - 1][y] == 'O' {
                can_move_up = false;
                can_move_right = true;
                cloned_matrix[x][y] = '>';
                continue;
            }
            cloned_matrix[x][y] = '.';
            x -= 1;
            cloned_matrix[x][y] = '^';

            if pos_map.contains_key(&(x, y)) && *pos_map.get(&(x, y)).unwrap() == '^' {
                return true;
            } 
            if !pos_map.contains_key(&(x, y)) {
                pos_map.insert((x, y), '^');
            }
        }
        else if can_move_right {
            if y + 1 >= cols {
                break;
            }
            if cloned_matrix[x][y + 1] == '#' || cloned_matrix[x][y + 1] == 'O' {
                can_move_right = false;
                can_move_down = true;
                cloned_matrix[x][y] = 'v';
                continue;
            }
            cloned_matrix[x][y] = '.';
            y += 1;
            cloned_matrix[x][y] = '>';

            if pos_map.contains_key(&(x, y)) && *pos_map.get(&(x, y)).unwrap() == '>' {
                return true;
            } 
            if !pos_map.contains_key(&(x, y)) {
                pos_map.insert((x, y), '>');
            }
        }
        else if can_move_down {
            if x + 1 >= rows {
                break;
            }
            if cloned_matrix[x + 1][y] == '#' || cloned_matrix[x + 1][y] == 'O' {
                can_move_down = false;
                can_move_left = true;
                cloned_matrix[x][y] = '<';
                continue;
            }
            cloned_matrix[x][y] = '.';
            x += 1;
            cloned_matrix[x][y] = 'v';

            if pos_map.contains_key(&(x, y)) && *pos_map.get(&(x, y)).unwrap() == 'v' {
                return true;
            } 
            if !pos_map.contains_key(&(x, y)) {
                pos_map.insert((x, y), 'v');
            }
        }
        else if can_move_left {
            if y == 0 {
                break;
            }
            if cloned_matrix[x][y - 1] == '#' || cloned_matrix[x][y - 1] == 'O' {
                can_move_left = false;
                can_move_up = true;
                cloned_matrix[x][y] = '^';
                continue;
            }
            cloned_matrix[x][y] = '.';
            y -= 1;
            cloned_matrix[x][y] = '<';
            if pos_map.contains_key(&(x, y)) && *pos_map.get(&(x, y)).unwrap() == '<' {
                return true;
            } 
            if !pos_map.contains_key(&(x, y)) {
                pos_map.insert((x, y), '<');
            }
        }

    }

    false
}

// Part 2
fn start_moving_(matrix: &mut Vec<Vec<char>>, 
                 x: &mut usize, 
                 y: &mut usize, 
                 rows: usize, 
                 cols: usize) -> () {
    
    let mut res = 0;
    for r in 0..rows {
        for c in 0..cols {
            println!("i: {}, j: {}", r, c);
            // place new obstruction if i can
            if matrix[r][c] == '.' {
                matrix[r][c] = 'O';
                let mut p_map: HashMap<(usize, usize), char> = HashMap::new();

                // check if this new obstruction gives us a loop
                let mut c_m = matrix.clone(); // deep copy
                if has_loop(&mut c_m, &mut p_map, x, y) {
                    res += 1;
                }
                // reset
                matrix[r][c] = '.';
            }
        }
    }
    println!("{}", res);
}

fn main() -> io::Result<()> {
    let file = File::open("day6input.txt")?;
    let reader = BufReader::new(file);
    
    let mut matrix: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let subarr: Vec<char> = line?.chars().collect();
        matrix.push(subarr);
    }
    
    // Part 1
    // Create new vector to have positions
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut mm:  Vec<Vec<char>> = vec![vec!['.'; cols]; rows];

    // find initial position
    let (mut x, mut y) = find_initial_position(&matrix);
    
    //print_matrix(&matrix);

    println!("");
    
    mm[x][y] = '#';

    // Part 1
    //start_moving(&mut matrix, &mut mm, &mut x, &mut y, rows, cols);

    // Part 2
    start_moving_(&mut matrix, &mut x, &mut y, rows, cols);

    Ok(())
}
