use std::io;

fn main() {
    let mut x = 1;
    let mut i = 1;
    let mut input = String::new();

    loop {
        println!("x is {}", x);
        x += 1;
        if x > 5 {
            break;
        }
    }

    while i < 5 {
        println!("i: {}", i);
        i += 1;
    }

    while input.trim() != "stop" {
        input.clear();
        println!("please enter a word, 'stop' to exit");
        println!();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        println!("you entered: {}", input);
    }

    // 1..=10 including 10
    for i in 1..10 {
        println!("counting {}",i);
    }
   
    // reverse
    for i in (1..10).rev() {
        println!("reverse {}",i);
    }

    //vector macro
    let nums = vec![1,2,3,4,5];
    for i in nums {
        println!("vec: {}",i)
    }

    println!("goodbye");
}
