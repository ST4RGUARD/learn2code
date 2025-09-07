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

fn threads() {
    use std::thread;
    let handle = thread::spawn(|| {
        println!("Running in a thread, again!");
    });
    
    handle.join().unwrap();
}

// pass messages between threads
fn channels() {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let msg = String::from("Polo");
        tx.send(msg).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Marco: {}", received);
}

// share ownership of data across threads immutably share bewteen threads
fn arc() {
    use std::sync::Arc;
    use std::thread;

    let data = Arc::new(5);
    
    for _ in 0..3 {
        let data_shared = data.clone();
        thread::spawn(move || {
            println!("{:?}", data_shared); 
        });
    }
}

// rayon is a parallelization lib that launches threads to speeg up certain operations
fn rayon() {
    use rayon::prelude::*;

    let data = vec![1, 2, 3];

    let parallel_sum: i32 = data.par_iter() // Specify the type
        .map(|x| x * x)
        .sum();

    println!("Parallel sum: {}", parallel_sum);
}

fn main() {
    ownership();
    borrowing();
    lifetimes();
    mutex();
    threads();
    channels();
    arc();
    rayon();
}
