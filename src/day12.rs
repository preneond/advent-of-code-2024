use std::collections::{HashMap, HashSet};

/// --- Day 12: Garden Groups ---

/// Depth-first search to find connected components (regions) of the same plant type.
fn dfs(
    garden: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    position: (usize, usize),
) -> Vec<(usize, usize)> {
    let (start_i, start_j) = position;
    let mut stack = vec![(start_i, start_j)];
    let mut component = vec![];

    while let Some((ci, cj)) = stack.pop() {
        if visited[ci][cj] {
            continue;
        }
        visited[ci][cj] = true;
        component.push((ci, cj));

        // Check neighbors (up, down, left, right)
        let neighbors = [
            (ci.wrapping_sub(1), cj),
            (ci + 1, cj),
            (ci, cj.wrapping_sub(1)),
            (ci, cj + 1),
        ];

        for &(ni, nj) in &neighbors {
            if ni < garden.len()
                && nj < garden[0].len()
                && !visited[ni][nj]
                && garden[ni][nj] == garden[ci][cj]
            {
                stack.push((ni, nj));
            }
        }
    }

    component
}

// ---------------------------------------------------------------------------
// PART ONE: Perimeter-Based Fence Price
// ---------------------------------------------------------------------------

/// Computes the fence price for a region by using `area * perimeter`.
/// The perimeter is counted as the total number of edges that border
/// out-of-bounds or a different region type.
fn compute_perimeter_price(region: &Vec<(usize, usize)>, garden: &Vec<Vec<char>>) -> i32 {
    let area = region.len(); // number of cells
    let mut perimeter = 0;

    let rows = garden.len();
    let cols = garden[0].len();

    // Put all cells of this region into a HashSet for quick lookup
    let region_set: HashSet<(usize, usize)> = region.iter().copied().collect();

    // For each cell in the region, check its 4 neighbors.
    // If the neighbor is out of bounds or not in the same region, perimeter++
    for &(r, c) in &region_set {
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            // Out of bounds => perimeter edge
            if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                perimeter += 1;
            }
            // Different region => perimeter edge
            else if !region_set.contains(&(nr as usize, nc as usize)) {
                perimeter += 1;
            }
        }
    }

    (area * perimeter) as i32
}

/// Finds connected components (regions) in `garden` and computes the total fence
/// price using the perimeter-based cost.
fn part_one(garden: &Vec<Vec<char>>) {
    let rows = garden.len();
    let cols = garden[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut components = Vec::new();

    // Identify all connected regions via DFS
    for i in 0..rows {
        for j in 0..cols {
            if !visited[i][j] {
                let component = dfs(garden, &mut visited, (i, j));
                components.push(component);
            }
        }
    }

    // Sum perimeter-based fence prices
    let total_price: i32 = components
        .iter()
        .map(|region| compute_perimeter_price(region, garden))
        .sum();

    println!("Part one: {}", total_price);
}

// ---------------------------------------------------------------------------
// PART TWO: Bulk Discount Fence Price
// ---------------------------------------------------------------------------

/// Counts how many "horizontal runs" exist among the given cells.
/// Each consecutive run of columns in the same row => 1 side.
fn count_horizontal_runs(cells: &[(usize, usize)]) -> usize {
    let mut rows_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(r, c) in cells {
        rows_map.entry(r).or_default().push(c);
    }

    let mut total = 0;
    for (_row, mut cols) in rows_map {
        cols.sort_unstable();
        total += count_runs_in_sorted_list(&cols);
    }
    total
}

/// Counts how many "vertical runs" exist among the given cells.
/// Each consecutive run of rows in the same column => 1 side.
fn count_vertical_runs(cells: &[(usize, usize)]) -> usize {
    let mut cols_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(r, c) in cells {
        cols_map.entry(c).or_default().push(r);
    }

    let mut total = 0;
    for (_col, mut rows) in cols_map {
        rows.sort_unstable();
        total += count_runs_in_sorted_list(&rows);
    }
    total
}

/// Counts consecutive-value runs in a sorted list.
/// E.g. [2,3,4,7,8,10] => 3 runs: [2..=4], [7..=8], [10..=10].
fn count_runs_in_sorted_list(sorted_vals: &[usize]) -> usize {
    if sorted_vals.is_empty() {
        return 0;
    }

    let mut runs = 1;
    let mut last = sorted_vals[0];
    for &v in &sorted_vals[1..] {
        if v != last + 1 {
            runs += 1;
        }
        last = v;
    }
    runs
}

/// Computes the fence price using the bulk-discount rule: `area * number_of_sides`.
/// Here, each continuous straight line (top, bottom, left, right) is counted as 1 side.
fn compute_bulk_fence_price(region: &[(usize, usize)], garden: &Vec<Vec<char>>) -> i32 {
    let area = region.len();
    let rows = garden.len();
    let cols = garden[0].len();

    // Identify which cells have top/bottom/left/right edges
    let top_edge_cells: Vec<(usize, usize)> = region
        .iter()
        .copied()
        .filter(|&(r, c)| r == 0 || garden[r - 1][c] != garden[r][c])
        .collect();

    let bottom_edge_cells: Vec<(usize, usize)> = region
        .iter()
        .copied()
        .filter(|&(r, c)| r == rows - 1 || garden[r + 1][c] != garden[r][c])
        .collect();

    let left_edge_cells: Vec<(usize, usize)> = region
        .iter()
        .copied()
        .filter(|&(r, c)| c == 0 || garden[r][c - 1] != garden[r][c])
        .collect();

    let right_edge_cells: Vec<(usize, usize)> = region
        .iter()
        .copied()
        .filter(|&(r, c)| c == cols - 1 || garden[r][c + 1] != garden[r][c])
        .collect();

    // Count straight runs in horizontal and vertical directions
    let top_sides    = count_horizontal_runs(&top_edge_cells);
    let bottom_sides = count_horizontal_runs(&bottom_edge_cells);
    let left_sides   = count_vertical_runs(&left_edge_cells);
    let right_sides  = count_vertical_runs(&right_edge_cells);

    let total_sides = top_sides + bottom_sides + left_sides + right_sides;
    // println!("Bulk region '{}' => area={} sides={}", garden[region[0].0][region[0].1], area, total_sides);

    (area * total_sides) as i32
}

/// Finds connected components (regions) in `garden` and computes the total fence
/// price using the bulk discount cost.
fn part_two(garden: &Vec<Vec<char>>) {
    let rows = garden.len();
    let cols = garden[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut components = Vec::new();

    // Identify all connected regions via DFS
    for i in 0..rows {
        for j in 0..cols {
            if !visited[i][j] {
                let component = dfs(garden, &mut visited, (i, j));
                components.push(component);
            }
        }
    }

    // Sum bulk-discount fence prices
    let total_price: i32 = components
        .iter()
        .map(|region| compute_bulk_fence_price(region, garden))
        .sum();

    println!("Part two: {}", total_price);
}

fn main() {
    // Parse the input map
    let garden: Vec<Vec<char>> = include_str!("../input/12.in")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    part_one(&garden); // perimeter-based price
    part_two(&garden); // bulk-discount price
}