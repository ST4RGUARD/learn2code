use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    //let args: Vec<String> = env::args().collect();
    let mut args = std::env::args();
    let program = args.next().unwrap_or("program".to_string());
    // this standard err checking isn't sufficient for 0 args 
    //if args.len() < 2 {
    //    panic!("Please provide a file name as an argument.");
    //}
    //
    let filepath = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: {} <filepath>", program);
        std::process::exit(1);
    });
    let file = File::open(&filepath);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("file not found! {}", error);
            }
            _ => {
                panic!("error opening file! {}", error);
            }
        },
    };
}
