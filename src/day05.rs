use std::collections::HashMap;
// --- Day 5: Print Queue ---

fn is_print_order_safe(print_order: &[i32], page_number_preds: &HashMap<i32, Vec<i32>>) -> bool {
    // Compare every pair (i, j) where j > i
    for i in 0..print_order.len() {
        let page_number = print_order[i];
        for j in i + 1..print_order.len() {
            let page_number_next = print_order[j];
            // If successor has a predecessor list
            if let Some(preds) = page_number_preds.get(&page_number_next) {
                // And if that list contains print_order[i], it's unsafe
                if preds.contains(&page_number) {
                    return false;
                }
            }
        }
    }
    true
}
fn part_one(print_orders: Vec<Vec<i32>>, page_number_preds: HashMap<i32, Vec<i32>>) {
    let safe_print_orders: Vec<&Vec<i32>> = print_orders
        .iter()
        .filter(|r| is_print_order_safe(r, &page_number_preds))
        .collect();

    let sum_of_middle: i32 = safe_print_orders
        .iter()
        .map(
            |order|
            order
                .get(order.len()/2)
                .unwrap_or(&0))
        .copied()
        .sum();

    println!("Part one: {:?}", sum_of_middle);
}


fn part_two(print_orders: Vec<Vec<i32>>, page_number_preds: HashMap<i32, Vec<i32>>) {
    let nonsafe_print_orders: Vec<&Vec<i32>> = print_orders
        .iter()
        .filter(|r| !is_print_order_safe(r, &page_number_preds))
        .collect();

    fn fix_order(order: &Vec<i32>, page_number_preds: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
        let mut fixed_order = order.clone();
        println!("old: {:?}", fixed_order);
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(fixed_order.len().saturating_sub(1)) {
                let j = i + 1;
                let page_number = fixed_order[i];
                let page_number_next = fixed_order[j];
                if let Some(preds) = page_number_preds.get(&page_number_next) {
                    if preds.contains(&page_number) {
                        fixed_order.swap(i, j);
                        swapped = true;
                    }
                }
            }
        }
        println!("new: {:?}", fixed_order);
        println!("###################");
        fixed_order
    }

    let fixed_nonsafe_print_orders: Vec<Vec<i32>> = nonsafe_print_orders
        .iter()
        .map(|order| fix_order(&order, &page_number_preds))
        .collect();

    let sum_of_middle: i32 = fixed_nonsafe_print_orders
        .iter()
        .map(
            |order|
            order
                .get(order.len()/2)
                .unwrap_or(&0))
        .copied()
        .sum();

    println!("Part two: {:?}", sum_of_middle);
}

fn main() {
    let lines: Vec<&str> = include_str!("../input/05.in").lines().collect();

    // Find the index of the empty line
    let empty_index = lines
        .iter()
        .position(|line| line.is_empty())
        .expect("No empty line found!");


    // 47|53 means that if an update includes both page number 47 and page number 53,
    // then page number 47 must be printed at some point before page number 53.
    let print_constraints: Vec<(i32, i32)> = lines[..empty_index]
        .iter()
        .map(|line| {
            let mut parts = line.split('|').map(|s| s.parse::<i32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    let print_orders: Vec<Vec<i32>> = lines[empty_index+1..]
        .iter()
        .map(
            |line| line
                .split(',')
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        )
        .collect();

    let mut page_number_preds: HashMap<i32, Vec<i32>> = HashMap::new();
    for (page_number, page_number_pred) in print_constraints {
        page_number_preds
            .entry(page_number)
            .or_insert_with(Vec::new)
            .push(page_number_pred);
    }

    part_one(print_orders.clone(), page_number_preds.clone());
    part_two(print_orders.clone(), page_number_preds.clone());
}
