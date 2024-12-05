use std::fs;
use std::path::Path;

// https://adventofcode.com/2024/day/4
pub fn run(input: &str) {
    let input_path = Path::new("src").join("day4").join(&input);
    let file_input = fs::read_to_string(input_path).expect("Couldn't read file");
    let lines = file_input.lines();

    let grid: Vec<String> = lines.map(|line| line.to_string()).collect();
    let mut points: Vec<Point> = Vec::new();
    for (row, line) in grid.iter().enumerate() {
        for (column, c) in line.chars().enumerate() {
            if c == 'X' {
                points.push(Point::new(column, row));
            }
        }
    }

    let result: i32 = points
        .iter()
        .map(|p| p.num_matches("XMAS".to_string(), &grid))
        .sum();

    println!("Part 1");
    println!("Result: {:?}", result);
}

#[derive(Debug)]
struct Point {
    column: usize,
    row: usize,
}

impl Point {
    fn new(column: usize, row: usize) -> Point {
        Point { column, row }
    }

    fn num_matches(&self, value: String, grid: &Vec<String>) -> i32 {
        if self.column > grid[0].len() - 1 || self.row > grid.len() - 1 {
            return 0;
        }

        let mut matches = 0;

        if self.matches_up(&value, &grid) {
            matches += 1;
        }
        if self.matches_ur(&value, &grid) {
            matches += 1;
        }
        if self.matches_right(&value, &grid) {
            matches += 1;
        }
        if self.matches_dr(&value, &grid) {
            matches += 1;
        }
        if self.matches_down(&value, &grid) {
            matches += 1;
        }
        if self.matches_dl(&value, &grid) {
            matches += 1;
        }
        if self.matches_left(&value, &grid) {
            matches += 1;
        }
        if self.matches_ul(&value, &grid) {
            matches += 1;
        }

        matches
    }

    fn matches_up(&self, value: &str, grid: &Vec<String>) -> bool {
        let match_len = value.len();
        if self.row.checked_sub(match_len - 1).is_some() {
            let mut up_value = String::new();
            for i in 0..match_len {
                up_value.push(grid[self.row - i].chars().nth(self.column).unwrap());
            }
            if up_value == value {
                return true;
            }
        }
        false
    }

    fn matches_ur(&self, value: &str, grid: &Vec<String>) -> bool {
        let match_len = value.len();
        if self.column + match_len <= grid[0].len() && self.row.checked_sub(match_len - 1).is_some()
        {
            let mut ur_value = String::new();
            for i in 0..match_len {
                ur_value.push(grid[self.row - i].chars().nth(self.column + i).unwrap());
            }
            if ur_value == value {
                return true;
            }
        }
        false
    }

    fn matches_right(&self, value: &str, grid: &Vec<String>) -> bool {
        let match_len = value.len();
        if self.column + match_len <= grid[0].len() {
            let mut right_value = String::new();
            for i in 0..match_len {
                right_value.push(grid[self.row].chars().nth(self.column + i).unwrap());
            }
            if right_value == value {
                return true;
            }
        }
        false
    }

    fn matches_dr(&self, value: &str, grid: &Vec<String>) -> bool {
        let match_len = value.len();
        if self.column + match_len <= grid[0].len() && self.row + match_len <= grid.len() {
            let mut dr_value = String::new();
            for i in 0..match_len {
                dr_value.push(grid[self.row + i].chars().nth(self.column + i).unwrap());
            }
            if dr_value == value {
                return true;
            }
        }
        false
    }

    fn matches_down(&self, value: &str, grid: &Vec<String>) -> bool {
        let match_len = value.len();
        if self.row + match_len <= grid.len() {
            let mut down_value = String::new();
            for i in 0..match_len {
                down_value.push(grid[self.row + i].chars().nth(self.column).unwrap());
            }
            if down_value == value {
                return true;
            }
        }
        false
    }

    fn matches_dl(&self, value: &str, grid: &Vec<String>) -> bool {
        let match_len = value.len();
        if self.column.checked_sub(match_len - 1).is_some() && self.row + match_len <= grid.len() {
            let mut dl_value = String::new();
            for i in 0..match_len {
                dl_value.push(grid[self.row + i].chars().nth(self.column - i).unwrap());
            }
            if dl_value == value {
                return true;
            }
        }
        false
    }

    fn matches_left(&self, value: &str, grid: &Vec<String>) -> bool {
        let match_len = value.len();
        if self.column.checked_sub(match_len - 1).is_some() {
            let mut left_value = String::new();
            for i in 0..match_len {
                left_value.push(grid[self.row].chars().nth(self.column - i).unwrap());
            }
            if left_value == value {
                return true;
            }
        }
        false
    }

    fn matches_ul(&self, value: &str, grid: &Vec<String>) -> bool {
        let match_len = value.len();
        if self.column.checked_sub(match_len - 1).is_some()
            && self.row.checked_sub(match_len - 1).is_some()
        {
            let mut ul_value = String::new();
            for i in 0..match_len {
                ul_value.push(grid[self.row - i].chars().nth(self.column - i).unwrap());
            }
            if ul_value == value {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_grid() -> Vec<String> {
        vec![
            "abcde".to_string(),
            "fghij".to_string(),
            "klmno".to_string(),
            "pqrst".to_string(),
            "uvwxy".to_string(),
        ]
    }

    #[test]
    fn test_matches_up_success() {
        let point = Point::new(0, 2);
        assert_eq!(point.matches_up("kfa", &get_grid()), true);
    }

    #[test]
    fn test_matches_up_fail() {
        let point = Point::new(0, 2);
        assert_eq!(point.matches_up("abc", &get_grid()), false);
    }

    #[test]
    fn test_matches_up_overflow() {
        let point = Point::new(0, 2);
        assert_eq!(point.matches_up("kfab", &get_grid()), false);
    }

    #[test]
    fn test_matches_ur_success() {
        let point = Point::new(0, 2);
        assert_eq!(point.matches_ur("kgc", &get_grid()), true);
    }

    #[test]
    fn test_matches_ur_fail() {
        let point = Point::new(0, 2);
        assert_eq!(point.matches_ur("abc", &get_grid()), false);
    }

    #[test]
    fn test_matches_ur_overflow() {
        let point = Point::new(0, 2);
        assert_eq!(point.matches_ur("kgca", &get_grid()), false);
    }
}
