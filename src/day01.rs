// --- Day 1: Historian Hysteria ---
fn part_one(location_distances: Vec<(i32, i32)>) {
    // find the total distance between the left list and the right list,
    // add up the distances between all of the pairs you found

    // Example
    // 3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3

    // 2 + 1 + 0 + 1 + 2 + 5 = 11
    let mut dist_left: Vec<i32> = location_distances.iter().map(|&(a, _)| a).collect();
    let mut dist_right: Vec<i32> = location_distances.iter().map(|&(_, b)| b).collect();

    dist_left.sort();
    dist_right.sort();

    // zip dist_left and dist_right and make abs subtraction of them
    let dist_diff: Vec<i32> = dist_left.iter().zip(dist_right.iter()).map(|(a, b)| (a - b).abs()).collect();
    println!("Part 1: {}", dist_diff.iter().sum::<i32>());
}


fn part_two(location_distances: Vec<(i32, i32)>) {
    // Calculate a total similarity score by adding up each number in the left list
    // after multiplying it by the number of times that number appears in the right list.

    // Example
    // 3   4
    // 4   3
    // 2   5
    // 1   3
    // 3   9
    // 3   3
    // result 9 + 4 + 0 + 0 + 9 + 9 = 31


    // solution
    // 2. iterate over the left list and for each number, find the number of times it appears in the right list
    // 3. multiply the number by the number of times it appears in the right list
    // 4. add the result to the similarity score
    // 5. return the similarity score

    let mut similarity_score = 0;
    for &num in location_distances.iter() {
        let count = location_distances.iter().filter(|&x| x.1 == num.0).count();
        similarity_score += num.0 * count as i32;
    }

    println!("Part 2: {}", similarity_score);
}

fn main() {
    let input = include_str!("../input/01.in");
    // line is a tuple of two number separated by a space - (i32, i32)
    // make a vector of these tuples
    let location_distances: Vec<(i32, i32)> = input.lines().map(
        |line| {
            let mut split = line.split_whitespace();
            let a: i32 = split.next().unwrap().parse().unwrap();
            let b: i32 = split.next().unwrap().parse().unwrap();
            (a, b)
        }
    ).collect();

    part_one(location_distances.clone());
    part_two(location_distances.clone());

}