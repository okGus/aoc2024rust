use std::fs;

fn make_filesystem(diskmap: &str) -> Vec<Option<usize>> {
    let mut blocks: Vec<Option<usize>> = Vec::new();
    let mut is_file = true;
    let mut id = 0;

    for ch in diskmap.chars() {
        let x = ch.to_digit(10).unwrap() as usize;
        if is_file {
            blocks.extend(vec![Some(id); x]);
            id += 1;
            is_file = false;
        } else {
            blocks.extend(vec![None; x]);
            is_file = true;
        }
    }

    blocks
}

fn compact_file(mut arr: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut first_free = 0;

    // Find the first free space
    while arr[first_free].is_some() {
        first_free += 1;
    }

    let mut i = arr.len() - 1;

    // Find the last file block
    while arr[i].is_none() {
        i -= 1;
    }

    // Move file blocks to the leftmost free space
    while i > first_free {
        arr[first_free] = arr[i];
        arr[i] = None;

        // Move `i` to the next non-free block
        while arr[i].is_none() && i > 0 {
            i -= 1;
        }

        // Move `first_free` to the next free space
        while arr[first_free].is_some() && first_free < arr.len() {
            first_free += 1;
        }
    }

    arr
}

fn file_checksum(arr: &Vec<Option<usize>>) -> usize {
    arr.iter()
        .enumerate()
        .filter_map(|(i, &x)| x.map(|val| i * val))
        .sum()
}

fn main() {
    let line = fs::read_to_string("day9input.txt")
        .expect("Failed to read file")
        .trim()
        .to_string();

    let filesystem = make_filesystem(&line);
    let compacted = compact_file(filesystem);
    let checksum = file_checksum(&compacted);

    println!("{}", checksum);
}

