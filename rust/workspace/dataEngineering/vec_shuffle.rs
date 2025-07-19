/*
This code starts with an initial VecDeque,
converts it to a Vec for shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "Cherry"
to the back of the deque. Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop from both ends
of the queue.

should we start with Vec and convert it to VecDeque?
since we are pushing back initially
*/

use rand::seq::SliceRandom; // rand is a random number generation library in Rust
use rand::rng;
use std::collections::VecDeque;
use std::time::Instant;

fn printFruitSalad(fruit: &VecDeque<&str>) {
    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}

fn shuffleVecDeque() {
    let mut fruit: Vec<&str> = Vec::new();
    fruit.push("Arbutus");
    fruit.push("Loquat");
    fruit.push("Strawberry Tree Berry");
    
    let mut rng = rng();
    let mut fruit = VecDeque::from(fruit);
    let (slice1, slice2) = fruit.as_mut_slices();
    slice1.shuffle(&mut rng);
    slice2.shuffle(&mut rng);

    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");
    printFruitSalad(&fruit);
}

fn shuffleVec() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry Tree Berry");

    let mut rng = rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    fruit.push_front("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_back("Cherry");
    printFruitSalad(&fruit);
}

fn main() {
    // -- Test Method 1 --
    // the VecDeque is shuffled in two slices - much slower than the Vec method 2
    let start1 = Instant::now();
    let result1 =  shuffleVecDeque();
    let duration1 = start1.elapsed();

    println!("Method 1 result: {:?}", result1);
    println!("Time taken: {:?}", duration1);

    // -- Test Method 2 --
    let start2 = Instant::now();
    let result2 = shuffleVec();
    let duration2 = start2.elapsed();

    println!("Method 2 result: {:?}", result2);
    println!("Time taken: {:?}", duration2);
}
