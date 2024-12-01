use std::{fs::read_to_string, iter::zip};

fn main() {
    if let Ok(text) = read_to_string("input.txt") {
        let (left_list, right_list) = parse(&text);
        let left_list = string_vec_to_int_vec(left_list);
        let right_list = string_vec_to_int_vec(right_list);

        println!("The result is: {}", calculate_result(left_list, right_list));
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

    return (left_list, right_list);
}

fn string_vec_to_int_vec(x: Vec<&str>) -> Vec<i32> {
    return x
        .into_iter()
        .map(|x| {
            x.parse::<i32>()
                .unwrap_or_else(|_err| panic!("That is NOT a number !!!!!!!"))
        })
        .collect();
}

fn calculate_result(mut left_list: Vec<i32>, mut right_list: Vec<i32>) -> i32 {
    left_list.sort();
    right_list.sort();

    let mut result = 0;

    for (left, right) in zip(left_list, right_list) {
        result += (left - right).abs();
    }

    return result;
}
