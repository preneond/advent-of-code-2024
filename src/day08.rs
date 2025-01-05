use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use itertools::Itertools;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Signal {
    x: i32,
    y: i32,
    value: String, // Updated to handle multi-character IDs
}

fn is_position_in_bounds(x: i32, y: i32, width: i32, height: i32) -> bool {
    x >= 0 && x < width && y >= 0 && y < height
}

fn print_map(signals: &HashMap<String, Vec<Signal>>, antinodes: &HashSet<(i32, i32)>, width: i32, height: i32) {
    for i in 0..height {
        for j in 0..width {
            let mut printed = false;
            for (_, value) in signals.iter() {
                for signal in value {
                    if signal.x == j && signal.y == i {
                        print!("{}", signal.value);
                        printed = true;
                        break;
                    }
                }
                if printed {
                    break;
                }
            }

            if !printed {
                if antinodes.contains(&(j, i)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}

fn part_one(signals: &HashMap<String, Vec<Signal>>, width: i32, height: i32) {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, value) in signals.iter() {
        if value.len() < 2 {
            continue;
        }

        let pairs: Vec<(&Signal, &Signal)> = value
            .iter()
            .combinations(2)
            .map(|x| (x[0], x[1]))
            .collect();

        for (a, b) in pairs {
            let norm_vector = (a.x - b.x, a.y - b.y);
            antinodes.insert((a.x + norm_vector.0, a.y + norm_vector.1));
            antinodes.insert((b.x - norm_vector.0, b.y - norm_vector.1));
        }
    }

    println!(
        "Part one: {}",
        antinodes
            .iter()
            .filter(|(x, y)| is_position_in_bounds(*x, *y, width, height))
            .count()
    );
}

fn part_two(signals: &HashMap<String, Vec<Signal>>, width: i32, height: i32) {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, value) in signals.iter() {
        if value.len() < 2 {
            continue;
        }

        let pairs: Vec<(&Signal, &Signal)> = value
            .iter()
            .combinations(2)
            .map(|x| (x[0], x[1]))
            .collect();

        for (a, b) in pairs {
            let norm_vector = (a.x - b.x, a.y - b.y);
            let mut antinode_a = (a.x + norm_vector.0, a.y + norm_vector.1);
            let mut antinode_b = (b.x - norm_vector.0, b.y - norm_vector.1);

            while is_position_in_bounds(antinode_a.0, antinode_a.1, width, height) {
                antinodes.insert(antinode_a.clone());
                antinode_a = (antinode_a.0 + norm_vector.0, antinode_a.1 + norm_vector.1);
            }

            while is_position_in_bounds(antinode_b.0, antinode_b.1, width, height) {
                antinodes.insert(antinode_b.clone());
                antinode_b = (antinode_b.0 - norm_vector.0, antinode_b.1 - norm_vector.1);
            }
        }
    }

    for (_, value) in signals.iter() {
        for signal in value {
            antinodes.insert((signal.x, signal.y));
        }
    }

    println!(
        "Part two: {}",
        antinodes
            .iter()
            .filter(|(x, y)| is_position_in_bounds(*x, *y, width, height))
            .count()
    );
}

fn main() {
    let lines: Vec<&str> = include_str!("../input/08.in").lines().collect();
    let mut signals: HashMap<String, Vec<Signal>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, value) in line.split_whitespace().enumerate() {
            let signal = Signal {
                x: x.try_into().unwrap(),
                y: y.try_into().unwrap(),
                value: value.to_string(),
            };
            signals.entry(value.to_string()).or_insert(Vec::new()).push(signal);
        }
    }

    let width: i32 = lines[0].split_whitespace().count().try_into().unwrap();
    let height: i32 = lines.len().try_into().unwrap();

    part_one(&signals, width, height);
    part_two(&signals, width, height);
}