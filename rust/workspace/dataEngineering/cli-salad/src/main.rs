use clap::Parser;
use cli_salad::make_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "jj",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts = Opts::parse_from(["program", "--number", "5"]);
    let num_fruits = opts.number;

    let salad = make_fruit_salad(num_fruits);

    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits, salad
    );
}
