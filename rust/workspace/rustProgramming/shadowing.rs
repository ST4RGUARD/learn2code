fn main() {
    let mut height = 50;
    height -= 20;

    // assigning condition result to variable
    let result = if height > 100 {
        "TALL"
    } else if height == 50 {
        "ME"
    } else if height <= 100 && height >= 40{
        "AVG"
    } else {
        "MINI"
    };

    println!("result: {}",result);
    // shadowing to a diff type
    let result = if height > 10 {true} else {false};
    println!("result: {}",result);
}
