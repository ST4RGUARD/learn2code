/*
Attempts to statistically decode a Caesar cipher.
Here's an example of how to use it:

This is a shift 16 message: "Off to the bunker. Every person for themselves"
"Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc"

cargo run -- --message "Ypp dy dro lexuob. Ofobi zobcyx pyb drowcovfoc" --guess

*/

use clap::Parser;
use decoder_ring::print_stats_analysis;

/// CLI tool to reverse engineer a Caesar cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    //message to decrypt
    #[arg(short, long)]
    cli_msg: Option<String>,

    //file to decrypt
    #[arg(short, long)]
    msg_file: Option<String>,

    //statistical information about the message
    #[arg(short, long)]
    stats: bool,

    //guess the shift
    #[arg(short, long)]
    guess: bool,
}

fn main() {
    let args = Args::parse();

    if args.cli_msg.is_none() && args.msg_file.is_none() {
        eprintln!("Please provide a message to decrypt using --message or --msg-file.");
        return;
    }

    if args.cli_msg.is_some() && args.msg_file.is_some() {
        eprintln!("Please provide only one of --message or --msg-file.");
        return;
    }

    let message = if let Some(msg) = args.cli_msg {
        msg
    } else {
        let file_path = args.msg_file.unwrap();
        std::fs::read_to_string(file_path)
            .expect("Failed to read the message file")
    };

    println!("Message to decrypt: {}", message);

    if args.stats {
        print_stats_analysis(&message);
    }
    
    if args.guess {
        let (depth, best_shift, decrypted, max_score) = decoder_ring::guess_shift(&message, 26);
        println!(
            "Best shift: {} (out of {}), score: {}",
            best_shift, depth, max_score
        );
        println!("Decrypted message: {}", decrypted);        
    }
}
