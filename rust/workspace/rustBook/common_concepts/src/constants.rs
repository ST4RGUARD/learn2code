pub fn constants() {
    // naming convention for constants is to use all uppercase with underscores between words
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");
    println!(
        "The value of MAX_POINTS * 3 Hours is {} seconds",
        MAX_POINTS * THREE_HOURS_IN_SECONDS
    );
}

