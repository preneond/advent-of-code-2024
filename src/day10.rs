use std::collections::HashSet;
use queues::*;

// --- Day 10: Hoof It ---
fn find_all_hiking_trail(
    trailhead: (usize, usize),
    topo_map: &[Vec<i32>],
) -> usize {
    // Use BFS to find all hiking trails starting from the trailhead
    let mut num_trails = 0;
    let mut q: Queue<(usize, usize)> = queue![];
    let mut visited = HashSet::new();

    q.add(trailhead).unwrap();
    visited.insert(trailhead);

    while let Ok(current) = q.remove() {
        if topo_map[current.0][current.1] == 9 {
            num_trails += 1;
            continue;
        }

        // Explore neighbors
        let neighbors = [
            (current.0.wrapping_sub(1), current.1), // Up
            (current.0 + 1, current.1),             // Down
            (current.0, current.1.wrapping_sub(1)), // Left
            (current.0, current.1 + 1),             // Right
        ];

        for &(nx, ny) in &neighbors {
            if nx < topo_map.len()
                && ny < topo_map[0].len()
                && topo_map[nx][ny] == topo_map[current.0][current.1] + 1
                && !visited.contains(&(nx, ny))
            {
                q.add((nx, ny)).unwrap();
                visited.insert((nx, ny));
            }
        }
    }

    num_trails
}

fn part_one(topo_map: Vec<Vec<i32>>) {
    let mut trailheads = Vec::new();

    // Find all trailheads (positions with height 0)
    for i in 0..topo_map.len() {
        for j in 0..topo_map[i].len() {
            if topo_map[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }

    let mut total_trails = 0;

    // For each trailhead, calculate the number of unique reachable hiking trails
    for trailhead in trailheads {
        let num_trails = find_all_hiking_trail(trailhead, &topo_map);
        total_trails += num_trails;
        // println!(
        //     "Number of hiking trails from trailhead {:?}: {:?}",
        //     trailhead, num_trails
        // );
    }

    println!("Part one: Total trails = {:?}", total_trails);
}

fn main() {
    // Parse the input map
    let topo_map: Vec<Vec<i32>> = include_str!("../input/10.in")
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap_or(1000) as i32)
                .collect()
        })
        .collect();

    part_one(topo_map);
}