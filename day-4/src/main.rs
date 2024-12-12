use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("That file is FUCKED !!!!!!!!");
    let grid: Vec<Vec<char>> = input.split('\n').map(|s| s.chars().collect()).collect();
    let width = grid[0].len();
    let height = grid.len();

    let mut xmas_count = 0;
    
    // horizontal checks
    let lines: Vec<&str> = input.split('\n').collect();
    for line in &lines {
        xmas_count += line.matches("XMAS").count();
        xmas_count += line.matches("SAMX").count();
    }

    // vertical checks
    for i in 0..width {
        let line: String = lines.iter().map(|line| line.chars().nth(i).unwrap()).collect();
        xmas_count += line.matches("XMAS").count();
        xmas_count += line.matches("SAMX").count();
    }

    // diagonal checks
    let target: Vec<char> = "XMAS".chars().collect();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] != 'X' {
                continue;
            }
            // down-right
            if y + 3 < height && x + 3 < width
                && (1..=3).all(|i| grid[y + i][x + i] == target[i]) {
                    xmas_count += 1;
                }
            // down-left
            if y + 3 < height && x >= 3
                && (1..=3).all(|i| grid[y + i][x - i] == target[i]) {
                    xmas_count += 1;
                }
            // up-right
            if y >= 3 && x + 3 < width
            && (1..=3).all(|i| grid[y - i][x + i] == target[i]) {
                xmas_count += 1;
            }
            // up-left
            if y >= 3 && x >= 3
            && (1..=3).all(|i| grid[y - i][x - i] == target[i]) {
                xmas_count += 1;
            }
        }
    }

    println!("Part 1: {}", xmas_count);
}
