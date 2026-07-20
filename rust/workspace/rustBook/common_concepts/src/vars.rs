pub fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    // without the mut keyword, we cannot change the value of x
    x = 6;
    println!("The value of x is: {x}");
}
