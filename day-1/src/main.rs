#![warn(clippy::pedantic)]
use std::{collections::HashMap, fs::read_to_string, iter::zip};

fn main() {
    if let Ok(text) = read_to_string("input.txt") {
        let (left_list, right_list) = parse(&text);
        let left_list = string_vec_to_int_vec(left_list);
        let right_list = string_vec_to_int_vec(right_list);

        println!("The result for part 1 is: {}", calculate_result_part1(&left_list, &right_list));
        println!("The result for part 2 is: {}", calculate_result_part2(&left_list, &right_list));
    } else {
        println!("That file doesnt fucking exist !!!!!!");
    }
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut left_list = vec![];
    let mut right_list = vec![];

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        };
        if let Some((left, right)) = line.split_once("   ") {
            left_list.push(left);
            right_list.push(right);
        } else {
            panic!("That file is fucked up and malformed !!!!!!!");
        }
    }

    (left_list, right_list)
}

fn string_vec_to_int_vec(x: Vec<&str>) -> Vec<i32> {
    x
        .into_iter()
        .map(|x| {
            x.parse::<i32>()
                .unwrap_or_else(|_err| panic!("That is NOT a number !!!!!!!"))
        })
        .collect()
}

fn calculate_result_part1(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let mut left_sorted = left_list.clone(); left_sorted.sort_unstable();
    let mut right_sorted = right_list.clone(); right_sorted.sort_unstable();

    let mut result = 0;

    for (left, right) in zip(left_list, right_list) {
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

    // count occurences on right
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