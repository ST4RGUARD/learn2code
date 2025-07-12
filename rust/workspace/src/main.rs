use std::io;

fn main() {
    let mut input = String::new();
    while input.trim() != "stop" {
        println!("Please enter a word (type 'stop' to exit):");
        io::stdin().read_line(&mut input).expect("failed to read input");
        println!("you entered: {}",input);
    }
    println!("goodbye");
}
