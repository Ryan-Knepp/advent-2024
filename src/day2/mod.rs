use std::fs;
use std::path::Path;

// https://adventofcode.com/2024/day/2
pub fn run(input: &str) {
    let input_path = Path::new("src").join("day2").join(&input);
    let file_input = fs::read_to_string(input_path).expect("Couldn't read file");
    let lines = file_input.lines();

    let mut num_safe_reports = 0;

    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        if is_safe(None, numbers[0], &numbers[1..]) {
            num_safe_reports += 1;
        }
    }

    println!("Part 1");
    println!("Result: {}", num_safe_reports);
}

enum Direction {
    Increasing,
    Decreasing,
}

fn is_safe(dir: Option<Direction>, current: i32, remaining: &[i32]) -> bool {
    let next = remaining[0];
    if current == next {
        return false;
    }
    if (current - next).abs() > 3 {
        return false;
    }
    let last_check = remaining.len() == 1;
    match dir {
        Some(Direction::Increasing) => {
            if current < next {
                if last_check {
                    return true;
                }
                is_safe(dir, next, &remaining[1..])
            } else {
                false
            }
        }
        Some(Direction::Decreasing) => {
            if current > next {
                if last_check {
                    return true;
                }
                is_safe(dir, next, &remaining[1..])
            } else {
                false
            }
        }
        None => {
            let new_dir = match current < next {
                true => Some(Direction::Increasing),
                false => Some(Direction::Decreasing),
            };
            is_safe(new_dir, next, &remaining[1..])
        }
    }
}
