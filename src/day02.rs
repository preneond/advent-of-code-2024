// --- Day 1: Historian Hysteria ---

fn is_safe(report: &Vec<i32>) -> bool {
    // check if all increasing
    // check if all decreasing
    // if not either, return false

    // do a reduce to make a diff of all consecutive numbers

    let mut diffs = report.windows(2).map(|w| w[1] - w[0]);
    // check if the sequence is all positive or all negative
    let is_increasing = diffs.clone().all(|d| d > 0);
    let is_decreasing = diffs.clone().all(|d| d < 0);
    let is_diff_between_1_and_3 = diffs.all(|d| d.abs() >= 1 && d.abs() <= 3);
    return (is_increasing || is_decreasing) && is_diff_between_1_and_3;
}
fn part_one(reports: Vec<Vec<i32>>) {
    let safe_reports = reports.iter().filter(|&r| is_safe(r));
    println!("Part one: {}", safe_reports.count());
}

fn part_two(reports: Vec<Vec<i32>>) {

    fn is_safe_2(report: &Vec<i32>) -> bool {
        let mut diffs = report.windows(2).map(|w| w[1] - w[0]);
        // check if the sequence is all positive or all negative
        let is_increasing = diffs.clone().all(|d| d > 0);
        let is_decreasing = diffs.clone().all(|d| d < 0);
        let is_diff_between_1_and_3 = diffs.all(|d| d.abs() >= 1 && d.abs() <= 3);
        if (is_increasing || is_decreasing) && is_diff_between_1_and_3 {
            return true;
        }
        // check if removing any level would make the report safe
        for i in 0..report.len() {
            let mut report_copy = report.clone();
            report_copy.remove(i);
            if is_safe(&report_copy) {
                return true;
            }
        }
        return false;
    }

    let safe_reports = reports.iter().filter(|&r| is_safe_2(r));
    println!("Part two: {}", safe_reports.count());

}

fn main() {
    let input = include_str!("../input/02.in");
    // line is a tuple of two number separated by a space - (i32, i32)
    // make a vector of these tuples
    let reports: Vec<Vec<i32>> = input.lines().map(
        |r| r.split_whitespace().map(|x| x.parse().unwrap()).collect()
    ).collect();

    part_one(reports.clone());
    part_two(reports.clone());

}