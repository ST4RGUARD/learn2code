fn ownership() {
    // x is owned by this scope
    {
        let _x = 5;
    }
    // Error! x no longer accessible here as it is now out of scope
}

fn borrowing() {
    // Create and own variable
    let x = 5;

    // Borrow a reference to x
    let y = &x;

    println!("x is: {}", x);
    println!("y is: {}", y);
}

fn lifetimes() {
    // Import threads
    use std::thread;

    // Define thread task
    let handle = thread::spawn(|| {
        println!("This runs in a different thread!!!");
    });

    // Join spawned thread
    handle.join().unwrap();
}

fn mutex() {
    // Import mutex
    use std::sync::Mutex;

    // Create mutex resource
    let m = Mutex::new(5);

    {
        // Lock mutex
        let mut num = m.lock().unwrap();

        // Modify
        *num = 1000;
    }

    println!("m = {:?}", m);
}

fn main() {
    ownership();
    borrowing();
    lifetimes();
    mutex();
}
