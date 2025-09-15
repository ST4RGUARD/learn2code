use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut numbers = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            break;
        }
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Not a number"))
            .collect();
        numbers.extend(nums);
    }

    let count = numbers[0];
    println!("count: {}", count);

}
