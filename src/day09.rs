// --- Day 9: Disk Fragmenter ---

fn part_one(mut memory_map: Vec<String>) {
    let mut first_free_idx = memory_map.iter().position(|c| c == ".").unwrap_or(memory_map.len());
    let mut last_full_idx = memory_map.iter().rposition(|c| c != ".").unwrap_or(0);

    while first_free_idx < last_full_idx {
        // Swap the elements
        memory_map.swap(first_free_idx, last_full_idx);

        // Find the next free and full indices
        first_free_idx = memory_map.iter().position(|c| c == ".").unwrap_or(memory_map.len());
        last_full_idx = memory_map.iter().rposition(|c| c != ".").unwrap_or(0);
    }

    // Calculate checksum
    println!("Part one: {}", compute_checksum(memory_map));
}

fn part_two(mut memory_map: Vec<String>) {
    // Attempt to move whole files to the leftmost span of free space blocks that could fit the file
    // Files are moved in decreasing order of file ID, starting with the highest
    // If no span of free space to the left is large enough, the file does not move

    let num_of_files = memory_map
        .iter()
        .filter_map(|c| c.parse::<usize>().ok())
        .max()
        .unwrap();

    for file_id in (0..=num_of_files).rev() {
        // Find the starting index and size of the current file
        if let Some(file_start_idx) = memory_map.iter().position(|c| c == &file_id.to_string()) {
            let file_size = memory_map
                .iter()
                .skip(file_start_idx)
                .take_while(|&c| c == &file_id.to_string())
                .count();

            // Search for the nearest free block to the left of the file
            let mut first_free_idx = memory_map.iter().position(|c| c == ".").unwrap_or(memory_map.len());

            while first_free_idx < file_start_idx {
                let free_mem_size = memory_map
                    .iter()
                    .skip(first_free_idx)
                    .take_while(|&c| c == ".")
                    .count();

                if free_mem_size >= file_size {
                    // Move the file to the free memory (left side)
                    memory_map.splice(
                        first_free_idx..first_free_idx + file_size,
                        vec![file_id.to_string(); file_size],
                    );

                    // Clear the original positions of the file
                    memory_map
                        .splice(file_start_idx..file_start_idx + file_size, vec![".".to_string(); file_size]);

                    break;
                }

                // Move to the next free block of memory
                first_free_idx += free_mem_size;
                first_free_idx += memory_map
                    .iter()
                    .skip(first_free_idx)
                    .take_while(|&c| c != ".")
                    .count();
            }
        }
    }

    // Debug output or further processing
    // println!("Resulting memory map: {:?}", memory_map.concat());
    println!("Part two: {:?}", compute_checksum(memory_map));
}

fn compute_checksum(memory_map: Vec<String>) -> usize {
    let checksum: usize = memory_map
        .iter()
        .enumerate()
        .filter_map(|(i, c)| c.parse::<usize>().ok().map(|file_id| i * file_id))
        .sum();

    return checksum
}


fn main() {
    let line: &str = include_str!("../input/09.in");
    let mut memory_map: Vec<String> = Vec::new();
    let mut file_id = 0;
    let mut chars = line.chars().peekable();

    while let Some(file_size_char) = chars.next() {
        // Parse file size
        let file_size: usize = file_size_char.to_digit(10).unwrap() as usize;

        // Parse free memory size if it exists
        let free_mem: usize = chars.peek()
            .and_then(|c| c.to_digit(10))
            .map(|d| {
                chars.next(); // Consume the character
                d as usize
            })
            .unwrap_or(0);

        // Add file blocks to the memory map
        memory_map.extend(vec![file_id.to_string(); file_size]);

        // Add free space blocks to the memory map
        memory_map.extend(vec![".".to_string(); free_mem]);

        // Increment the file ID
        file_id += 1;
    }

    // part_one(memory_map.clone());
    part_two(memory_map.clone());
}