use regex::Regex;
use std::fs;
use std::path::Path;

// https://adventofcode.com/2024/day/3
pub fn run(input: &str) {
    let input_path = Path::new("src").join("day3").join(&input);
    let file_input = fs::read_to_string(input_path).expect("Couldn't read file");

    let re = Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)").unwrap();
    let mul_results: Vec<i32> = re
        .captures_iter(&file_input)
        .map(|caps| {
            let left: i32 = caps["left"].parse().unwrap();
            let right: i32 = caps["right"].parse().unwrap();
            left * right
        })
        .collect();
    let result = mul_results.iter().sum::<i32>();

    println!("Part 1");
    println!("Result: {:?}", result);
}
