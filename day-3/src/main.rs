use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let input = read_to_string("input.txt").unwrap();
    let mut total = 0;
    for c in pattern.captures_iter(&input) {
        let lhs: i32 = c[1].parse().unwrap();
        let rhs: i32 = c[2].parse().unwrap();
        total += lhs * rhs;
    }
    println!("The answer is: {}", total);
}
