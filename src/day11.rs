use std::collections::HashMap;
use rayon::prelude::*; // Import Rayon for parallel iterators

// --- Day 11: Plutonian Pebbles ---

fn part_one(stones: Vec<i64>, num_blinkings: i32) {
    let mut stones_tmp = stones.clone();
    for _ in 0..num_blinkings {
        let mut stones_tmp_new: Vec<i64> = Vec::new();
        for stone in &stones_tmp {
            // If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
            let stone_str = stone.abs().to_string();
            if *stone == 0 {
                stones_tmp_new.push(1);
            }
            // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
            // The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone
            else if stone_str.len() % 2 == 0 {
                let mid = stone_str.len() / 2;
                // split str into two
                let stone_first_half =  stone_str[..mid].parse::<i64>().unwrap();
                let stone_second_half = stone_str[mid..].parse::<i64>().unwrap();
                stones_tmp_new.push(stone_first_half);
                stones_tmp_new.push(stone_second_half);
            }
            // If none of the other rules apply, the stone is replaced by a new stone;
            // the old stone's number multiplied by 2024 is engraved on the new stone.
            else {
                stones_tmp_new.push(stone*2024)
            }
        }
        stones_tmp = stones_tmp_new.clone();
        // println!("After {:?} blink: {:?}", blinking, stones_tmp)

    }


    println!("Part one: {:?}", stones_tmp.len())
}

fn part_two(stones: Vec<i64>, num_blinkings: i32) {
    let mut stone_counts: HashMap<i64, u64> = HashMap::new();

    // Initialize the HashMap with the initial stones
    for stone in stones {
        *stone_counts.entry(stone).or_insert(0) += 1;
    }

    for blinking in 0..num_blinkings {
        println!("Blinking {} times", blinking);

        // Use Rayon for parallel processing
        let new_stone_counts: HashMap<i64, u64> = stone_counts
            .par_iter() // Parallel iterator over current stone_counts
            .map(|(&stone, &count)| {
                if stone == 0 {
                    // If the stone is 0, it transforms into a stone with value 1
                    vec![(1, count)]
                } else {
                    let abs_stone = stone.abs();
                    let num_digits = ((abs_stone as f64).log10() as u32) + 1; // Efficient digit counting

                    if num_digits % 2 == 0 {
                        // Split the stone into two halves
                        let half_power = 10i64.pow(num_digits / 2);
                        let stone_first_half = abs_stone / half_power;
                        let stone_second_half = abs_stone % half_power;

                        vec![(stone_first_half, count), (stone_second_half, count)]
                    } else {
                        // Multiply the stone
                        let new_value = stone * 2024;
                        vec![(new_value, count)]
                    }
                }
            })
            .flatten() // Flatten the Vec<Vec<(stone, count)>> into Vec<(stone, count)>
            .fold(HashMap::new, |mut acc, (stone, count)| {
                // Combine results into a HashMap
                *acc.entry(stone).or_insert(0) += count;
                acc
            })
            .reduce(HashMap::new, |mut acc, partial| {
                // Merge partial results
                for (stone, count) in partial {
                    *acc.entry(stone).or_insert(0) += count;
                }
                acc
            });

        stone_counts = new_stone_counts;
    }

    // Compute the total number of stones
    println!("Part two: {:?}", stone_counts.iter().map(|(_, v)| *v).sum::<u64>());
}

fn main() {
    // Parse the input map
    let stones: Vec<i64> = include_str!("../input/11.in").split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

    // part_one(stones.clone(), 25);
    part_two(stones.clone(), 75);
    // part_two();
}
