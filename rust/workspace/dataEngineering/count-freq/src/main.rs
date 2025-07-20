use std::{collections::HashMap};
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "jj",
    about = "Number of fruits to include"
)]
struct Opts {
    // numbers
    // #[clap(short, long, num_args = 1..)]
    // numbers: Vec<i32>,
    // string
    #[clap(short, long)]
    sentence: String,
}

fn logic(numbers: String) -> Vec<(String, u32)> {
    let mut frequencies: HashMap<String, u32> = HashMap::new();

    for num in numbers.split_whitespace() {
        let frequency: &mut u32 = frequencies.entry(num.to_string()).or_insert(0);
        *frequency += 1;
    }

    let mut result: Vec<(String, u32)> = Vec::new();

    for (num , frequency ) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    let opts: Opts = Opts::parse();
    let words = opts.sentence;
    let result = logic(words);

    println!("The frequency of each number in the vector is: {:?}", result);
}
