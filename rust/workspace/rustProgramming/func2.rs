fn split_string(s: String, delimeter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimeter).collect();
    let result = parts.get(field);
    result.expect("something went wrong").to_string()
}

fn main() {
    let chunk = split_string("hello,world".to_string(),',',1);
    println!("Split String {}",chunk);
}
