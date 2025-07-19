use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("Cargo.lock");
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
