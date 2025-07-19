use std::io;

fn main() {
    //let name = "Hello";
    println!("enter greeting: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read input");


    match name.trim() {
        "Hi" => println!("ok hi"),
        "BYE" => println!("ok bye"),
        "Hello" => println!("konichiwa"),
        _ => println!("default"),
    }
}
