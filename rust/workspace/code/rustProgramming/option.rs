fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0{
        None
    } else {
        Some(x/y)
    }
}

fn main() {
    let a = 10;
    let b = 0;

    let result = divide(a,b);

    match result{
        Some(x) => println!("result: {}", x),
        None => println!("error: division by zero"),
    }
}
