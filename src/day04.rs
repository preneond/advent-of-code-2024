use std::ops::Add;

// --- Day 4: Ceres Search ---
fn collect_edge_diagonals(lines: &[String], min_len: usize) -> Vec<String> {
    let height = lines.len();
    if height == 0 {
        return Vec::new();
    }
    let width = lines[0].len();

    let mut results = Vec::new();

    // == DOWN-RIGHT (↘) ==

    // 1) Start from left edge: (row in 0..height, col = 0)
    for start_row in 0..height {
        let diagonal = gather_down_right(lines, start_row, 0);
        if diagonal.len() >= min_len {
            results.push(diagonal);
        }
    }

    // 2) Start from top edge: (row = 0, col in 0..width)
    //    (We already did col = 0 in the previous loop, so if you want
    //     to avoid duplicating the (0,0) start, do col in 1..width).
    for start_col in 1..width {
        let diagonal = gather_down_right(lines, 0, start_col);
        if diagonal.len() >= min_len {
            results.push(diagonal);
        }
    }

    // == DOWN-LEFT (↙) ==

    // 1) Start from right edge: (row in 0..height, col = width - 1)
    if width > 0 {
        for start_row in 0..height {
            let diagonal = gather_down_left(lines, start_row, width - 1);
            if diagonal.len() >= min_len {
                results.push(diagonal);
            }
        }
    }

    // 2) Start from top edge: (row = 0, col in 0..width)
    //    (Again, skip col=width-1 if you want to avoid duplication.)
    if height > 0 {
        for start_col in 0..width - 1 {
            let diagonal = gather_down_left(lines, 0, start_col);
            if diagonal.len() >= min_len {
                results.push(diagonal);
            }
        }
    }

    results
}

/// Gathers characters along a down-right diagonal (↘) starting at (row, col).
fn gather_down_right(lines: &[String], mut row: usize, mut col: usize) -> String {
    let height = lines.len();
    let width = lines[0].len();
    let mut result = String::new();

    while row < height && col < width {
        if let Some(ch) = lines[row].chars().nth(col) {
            result.push(ch);
        }
        row += 1;
        col += 1;
    }

    result
}

/// Gathers characters along a down-left diagonal (↙) starting at (row, col).
fn gather_down_left(lines: &[String], mut row: usize, mut col: usize) -> String {
    let height = lines.len();
    let width = lines[0].len();
    let mut result = String::new();

    while row < height && col < width {
        if let Some(ch) = lines[row].chars().nth(col) {
            result.push(ch);
        }

        // Move down-left
        if col == 0 {
            break;
        }
        row += 1;
        col -= 1;
    }

    result
}

fn part_one(lines: Vec<&str>) {
    let lines_orig: Vec<String> = lines.iter().map(|x| x.to_string()).collect();
    let lines_rev: Vec<String> = lines.iter().map(|x| x.chars().rev().collect()).collect();

    let mut lines_vertical: Vec<String> = Vec::new();
    for j in 0..lines[0].len() {
        let mut tmp_line = String::new();
        for i in 0..lines.len() {
            if let Some(c) = lines[i].chars().nth(j) {
                tmp_line = tmp_line.add(&c.to_string());
            }
        }
        lines_vertical.push(tmp_line);
    }
    let lines_vertical_rev= lines_vertical.iter().map(|x| x.chars().rev().collect()).collect();

    let lines_diagonal = collect_edge_diagonals(&lines_orig, 4);

    let lines_diagonal_rev = lines_diagonal.iter().map(|x| x.chars().rev().collect()).collect();

    let lines_all = vec![lines_orig, lines_rev, lines_vertical, lines_vertical_rev, lines_diagonal, lines_diagonal_rev];

    let mut ceres_count = 0;
    for tmp_lines in lines_all {
        for line in tmp_lines {
            for i in 0..line.len()-3 {
                if line[i..i+4].eq("XMAS") {
                    ceres_count += 1;
                    // println!("Found Ceres at line: {}", line);
                }
            }
        }
    }
    println!("Part one {:?}", ceres_count)
}

fn is_x_mas(lines: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // Check bounds to avoid out-of-bounds errors
    if i + 2 >= lines.len() || j + 2 >= lines[i].len() {
        return false;
    }

    // Top-left MAS can be either forward (MAS) or backward (SAM)
    let top_left_mas = (lines[i][j] == 'M' && lines[i + 1][j + 1] == 'A' && lines[i + 2][j + 2] == 'S') ||
        (lines[i][j] == 'S' && lines[i + 1][j + 1] == 'A' && lines[i + 2][j + 2] == 'M');

    // Bottom-left MAS can be either forward (MAS) or backward (SAM)
    let bottom_left_mas = (lines[i + 2][j] == 'M' && lines[i + 1][j + 1] == 'A' && lines[i][j + 2] == 'S') ||
        (lines[i + 2][j] == 'S' && lines[i + 1][j + 1] == 'A' && lines[i][j + 2] == 'M');

    // Both top-left and bottom-left MAS must be present to form an X-MAS
    top_left_mas && bottom_left_mas
}

fn part_two(lines: Vec<&str>) {
    let char_lines: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let mut x_mas_count = 0;

    for i in 0..char_lines.len() - 2 {
        for j in 0..char_lines[i].len() - 2 {
            if is_x_mas(&char_lines, i, j) {
                x_mas_count += 1;
            }
        }
    }

    println!("Part two {:?}", x_mas_count)
}

fn main() {
    let lines: Vec<&str> = include_str!("../input/04.in").lines().collect();
    part_one(lines.clone());
    part_two(lines.clone());
}
