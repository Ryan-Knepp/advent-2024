use std::fs;
use std::path::Path;

pub fn run(input: &str) {
    let input_path = Path::new("src").join("day1").join(&input);
    let file_input = fs::read_to_string(input_path).expect("Couldn't read file");
    let lines = file_input.lines();

    let number_pairs: Vec<(i32, i32)> = lines
        .map(|line| {
            let mut numbers = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());

            (numbers.next().unwrap(), numbers.next().unwrap())
        })
        .collect();
    let mut left: Vec<i32> = number_pairs.iter().map(|(a, _)| *a).collect();
    let mut right: Vec<i32> = number_pairs.iter().map(|(_, b)| *b).collect();

    left.sort();
    right.sort();

    let result = left.iter().enumerate().fold(0, |acc, (i, l)| {
        let diff = right[i] - l;
        acc + diff.abs()
    });

    println!("Part 1");
    println!("Result: {}", result);
}
