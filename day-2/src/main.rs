use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("That file DOESNT EXIST !!!!!!!!").trim().to_owned();

    let reports: Vec<Vec<i32>> = input
        .split('\n')
        .map(|s| {
            s.split(' ')
                .map(|s2| s2.parse().unwrap())
                .collect()
        })
        .collect();

    let mut num_safe = 0;
    for report in reports {
        let differences: Vec<i32> = report.windows(2).map(|s| (s[1] - s[0])).collect();
        let starting_direction = differences[0].signum();
        // if the first level isnt increasing or decreasing, it isn't safe
        if starting_direction == 0 { continue; }
        // if any levels differ in direction from the first, it isn't safe
        if differences[1..].iter().any(|x| x.signum() != starting_direction) { continue; }
        // if any difference is greater than 3, it isn't safe
        if differences.into_iter().any(|x| x.abs() > 3) { continue; }
        // otherwise, it's safe.
        num_safe += 1;
    }

    println!("The answer is: {}", num_safe);
}

