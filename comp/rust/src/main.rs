use std::{i32, io};

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut nums = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("Not a number"));

    let n = nums.next().unwrap();
    let m = nums.next().unwrap();


}
