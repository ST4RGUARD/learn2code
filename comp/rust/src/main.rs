use std::io;

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

fn read_usize() -> usize {
    read_line().trim().parse().unwrap()
}

fn read_vec_i32() -> Vec<i32> {
    read_line()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn solve() {
    let line1 = read_vec_i32();
    let line2 = read_vec_i32();

    println!("Line 1: {:?}", line1);
    println!("Line 2: {:?}", line2);
}

fn main() {
    // test cases
    let t = read_usize();

    for _ in 0..t {
        solve();
    }
}
