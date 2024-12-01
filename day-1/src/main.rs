#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs::read_to_string, iter::zip};

fn main() {
    if let Ok(text) = read_to_string("input.txt") {
        let (left_list, right_list) = parse(&text);

        println!("The result for part 1 is: {}", calculate_result_part1(&left_list, &right_list));
        println!("The result for part 2 is: {}", calculate_result_part2(&left_list, &right_list));
    } else {
        println!("That file doesnt fucking exist !!!!!!");
    }
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let numbers: Vec<i32> = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    return numbers.chunks(2).map(|chunk| (chunk[0], chunk[1])).unzip();
}

fn calculate_result_part1(left_list: &[i32], right_list: &[i32]) -> i32 {
    let mut left_sorted = left_list.to_owned(); left_sorted.sort_unstable();
    let mut right_sorted = right_list.to_owned(); right_sorted.sort_unstable();

    let mut result = 0;

    for (left, right) in zip(left_sorted, right_sorted) {
        result += (left - right).abs();
    }

    result
}

fn calculate_result_part2(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let mut count: HashMap<i32, i32> = HashMap::new();

    // get keys from left list
    for n in left_list {
        count.insert(*n, 0);
    }

    // count occurrences on right
    for n in right_list {
        if let Some(slot) = count.get_mut(n) {
            *slot += 1;
        }
    }

    let mut result = 0;

    for (k, v) in count {
        result += k * v;
    }

    result
}