
#[derive(Debug)]
enum FileSize {
    Bytes(f64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: f64, _unit: String) -> Vec<FileSize> {
    vec![
        FileSize::Bytes(size),
        FileSize::Kilobytes(size / 1000.0),
        FileSize::Megabytes(size / 1_000_000.0),
        FileSize::Gigabytes(size / 1_000_000_000.0),
    ]
}

fn get_input() -> (f64, String)  {
    let mut args = std::env::args();
    let program  = args.next().unwrap_or("program".to_string());

    let value = args.nth(0).unwrap_or_else(|| {
        println!("Usage: {} <number> <unit>",program);
        std::process::exit(1);
    });

    let unit = args.next().unwrap_or_else(|| {
        println!("Usage: {} <number> <unit>",program);
        std::process::exit(1);
    });

    let number = value.parse::<f64>().unwrap_or_else(|_| {
        println!("Please provide a valid integer for size");
        std::process::exit(1);
    });

    (number, unit.to_lowercase())
}

fn print_output(sizes: Vec<FileSize>) {
    println!("Sizes are {:?} !", sizes)
}

fn main() {
    let (size, unit) = get_input();
    let formatted    = format_size(size,unit);
    print_output(formatted);
}
