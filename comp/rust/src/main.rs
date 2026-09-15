use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn read_usize() -> usize {
    read_line().trim().parse().unwrap()
}

fn read_digits_i32() -> Vec<i32> {
    read_line()
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i32))
        .collect()
}

fn read_vec_i32() -> Vec<i32> {
    read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve() {
    // lines per test case
    let line1 = read_vec_i32();
    let farms = line1[0] / line1[1];

    let line2 = read_digits_i32();

    let cnt = line2
        .chunks(line1[1].try_into().unwrap())
        .filter(|chunk| chunk.iter().any(|&x| x == 0))
        .count();

    let result = farms - (cnt as i32);
    println!("{result}");
}

fn main() {
    // test cases
    let t = read_usize();

    for _ in 0..t {
        solve();
    }
}
