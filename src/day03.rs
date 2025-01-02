use regex::Regex;
// --- Day 3: Mull It Over ---

fn remove_sections_between_dont_and_do(text: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();

    // Whether we're skipping text (i.e., between don't() and the next do()).
    let mut skip_mode = false;

    while i < len {
        // Try to match "don't()" or "do()" at position i.
        if !skip_mode && text[i..].starts_with("don't()") {
            // We just encountered "don't()" -> enter skip mode.
            skip_mode = true;
            i += "don't()".len();
        } else if skip_mode && text[i..].starts_with("do()") {
            // We just encountered "do()" while skipping -> exit skip mode.
            skip_mode = false;
            i += "do()".len();
        } else {
            // We either copy (not skipping) or skip (skip_mode == true).
            if !skip_mode {
                result.push(chars[i]);
            }
            i += 1;
        }
    }
    result
}

fn mull_it_over(text: &str) -> i32 {
    let text_no_spaces = text.replace(" ", "");

    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    // 4. Initialize accumulator
    let mut mul_res: i32 = 0;

    // 5. Find all matches in the file and sum them up
    for mat in re.find_iter(&text_no_spaces) {
        // mat.as_str() is something like "mul(3,10)"
        let substring = mat.as_str();

        // Remove "mul(" (4 chars) and the trailing ")" (1 char):
        // e.g. "mul(3,10)" -> inside: "3,10"
        let inside = &substring[4..(substring.len() - 1)];

        // Split at the comma -> ["3", "10"]
        let parts: Vec<&str> = inside.split(',').collect();

        // Parse the two numbers
        let x: i32 = parts[0].parse().expect("Invalid integer");
        let y: i32 = parts[1].parse().expect("Invalid integer");

        // Multiply and accumulate
        mul_res += x * y;
    }
    return mul_res;
}

fn part_one(text: &str) {
    let res = mull_it_over(text);
    println!("Part one: {:?}", res);
}

fn part_two(text: &str) {
    let res = mull_it_over(remove_sections_between_dont_and_do(text).as_str());
    print!("Part two: {:?}", res);
}

fn main() {
    let input = include_str!("../input/03.in");
    // line is a tuple of two number separated by a space - (i32, i32)
    // make a vector of these tuples
    part_one(input.clone());
    part_two(input.clone());

}
