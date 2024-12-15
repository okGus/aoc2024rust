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

fn compact_file_whole(mut arr: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut file_positions: Vec<(usize, usize, usize)> = Vec::new(); // (file_id, start, length)
    let mut free_spans: Vec<(usize, usize)> = Vec::new(); // (start, length)

    // Identify file positions and free spans
    let mut i = 0;
    while i < arr.len() {
        if let Some(file_id) = arr[i] {
            let start = i;
            while i < arr.len() && arr[i] == Some(file_id) {
                i += 1;
            }
            let length = i - start;
            file_positions.push((file_id, start, length));
        } else {
            let start = i;
            while i < arr.len() && arr[i].is_none() {
                i += 1;
            }
            let length = i - start;
            free_spans.push((start, length));
        }
    }

    // Sort files by descending file ID
    file_positions.sort_by(|a, b| b.0.cmp(&a.0));

    // Attempt to move each file
    for (file_id, file_start, file_length) in file_positions {
        // Find the first free span that can fit the file
        if let Some((span_idx, (span_start, span_length))) = free_spans
            .iter()
            .enumerate()
            .find(|&(_, &(span_start, span_length))| span_length >= file_length && span_start < file_start)
        {
            // Move the file
            for i in 0..file_length {
                arr[span_start + i] = Some(file_id);
            }
            for i in file_start..file_start + file_length {
                arr[i] = None;
            }

            // Update the free spans
            let updated_span_start = span_start + file_length;
            let updated_span_length = span_length - file_length;
            if updated_span_length > 0 {
                free_spans[span_idx] = (updated_span_start, updated_span_length);
            } else {
                free_spans.remove(span_idx);
            }

            // Sort free spans by increasing start position
            free_spans.sort_by(|a, b| a.0.cmp(&b.0));
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
    let compacted = compact_file_whole(filesystem);
    let checksum = file_checksum(&compacted);

    println!("{}", checksum);
}

