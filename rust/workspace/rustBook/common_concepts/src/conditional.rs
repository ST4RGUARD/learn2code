pub fn conditional() {
    let condition = true;
    // usin if in a let statement
    // variables must have a single type, and Rust must know at compile time what var number is
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
