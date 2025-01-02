use regex::Regex;
// --- Day 1: Historian Hysteria ---

fn main() {
    let input = include_str!("../input/04.in");
    // line is a tuple of two number separated by a space - (i32, i32)
    // make a vector of these tuples
    part_one(input.clone());
    part_two(input.clone());

}
