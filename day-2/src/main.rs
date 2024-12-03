use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt")
        .expect("That file DOESNT EXIST !!!!!!!!")
        .trim()
        .to_owned();

    let reports: Vec<Vec<i32>> = input
        .split('\n')
        .map(|s| s.split(' ').map(|s2| s2.parse().unwrap()).collect())
        .collect();

    println!("The part 1 answer is: {}", part_1_solution(&reports));
    println!("The part 2 answer is: {}", part_2_solution(&reports));
}

fn part_1_solution(reports: &Vec<Vec<i32>>) -> i32 {
    let mut num_safe = 0;
    for report in reports {
        if check_report(report) {
            num_safe += 1;
        }
    }
    num_safe
}

fn part_2_solution(reports: &Vec<Vec<i32>>) -> i32 {
    let mut num_safe = 0;
    for report in reports {
        if check_report_tolerant(report) {
            num_safe += 1;
        }
    }
    num_safe
}

fn check_report_tolerant(report: &[i32]) -> bool {
    if check_report(report) {
        true
    } else {
        for i in 0..report.len() {
            let mut modified: Vec<i32> = report.to_vec();
            modified.remove(i);
            if check_report(&modified) {
                return true;
            }
        }
        false
    }
}

fn check_report(report: &[i32]) -> bool {
    let differences: Vec<i32> = report.windows(2).map(|s| (s[1] - s[0])).collect();
    let starting_direction = differences[0].signum();
    // if the first level isnt increasing or decreasing, it isn't safe
    if starting_direction == 0 {
        return false;
    }
    // if any levels differ in direction from the first, it isn't safe
    if differences[1..]
        .iter()
        .any(|x| x.signum() != starting_direction)
    {
        return false;
    }
    // if any difference is greater than 3, it isn't safe
    if differences.into_iter().any(|x| x.abs() > 3) {
        return false;
    }
    // otherwise, it's safe.
    true
}
