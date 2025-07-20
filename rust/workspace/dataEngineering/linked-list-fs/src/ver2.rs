/*
As with the VecDeque example, this code starts by creating a LinkedList of fruits,
converts it to a Vec for shuffling, and then converts it back to a LinkedList.
After the shuffling, it adds "Pomegranate", "Fig", and "Cherry" to the end of the list.
Finally, it prints out the final fruit salad.

This example shows how to use a LinkedList, but remember that LinkedList
has a higher memory overhead and worse cache locality than Vec or VecDeque,
so it's typically not the best choice unless you have a specific need for the properties 
of a linked list. In Rust, it's usually better to use a Vec or VecDeque.

A LinkedList is a doubly-linked list, which means that each element in the list
has a pointer to the next element and the previous element.
A great example of when to use a LinkedList is when you need to insert or remove elements
from the middle of the list.
*/

use rand::seq::SliceRandom;
use rand::prelude::*;
use std::collections::LinkedList;
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "2.0",
    author = "jj",
    about = "select random fruit in the list"
)]
struct Opts {
    // numbers
    // #[clap(short, long, num_args = 1..)]
    // numbers: Vec<i32>,
    // string
    #[clap(short, long)]
    word: String,
    // position to insert the fruit
    #[clap(short = 'p', long)]
    position: usize,
}

fn main() {
    let opts: Opts = Opts::parse();
    let fruit = opts.word;

    let mut fruits: LinkedList<&str> = LinkedList::new();
    fruits.push_back("Arbutus");
    fruits.push_back("Loquat");
    fruits.push_back("Strawberry Tree Berry");

    // Scramble (shuffle) the fruit
    // this is where we would add the user fruit normally
    let mut rng = rand::rng();
    let mut fruits: Vec<_> = fruits.into_iter().collect();
    fruits.shuffle(&mut rng);

    // here we will choose a random fruit
    // we can do it after we add the user fruit later but
    // we would need to convert the LinkedList to a Vec AGAIN
    if let Some(choice) = fruits.choose(&mut rng) {
        println!("Random choice: {}", choice);
    } else {
        println!("The list is empty");
    }

    // Convert it back to LinkedList
    // but we are going to add it to the LinkedList
    let mut fruits: LinkedList<_> = fruits.into_iter().collect();

    // Add fruits to the both ends of the list after shuffling
    fruits.push_front("Pomegranate");
    fruits.push_back("Fig");
    fruits.push_back("Cherry");

    // Print out the fruit salad
    println!("Current fruits:");
    for (i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            println!("{i} {item} ");
        } else {
            println!("{i} {item} ");
        }
    }
    println!("-------------------------------------");
    
    // user fruit added here
    let insert_pos = opts.position.min(fruits.len());
    let mut new_fruits = LinkedList::new();
    for (i, item) in fruits.iter().enumerate() {
        if i == insert_pos {
            new_fruits.push_back(fruit.as_str());
        }
        new_fruits.push_back(item);
    }
    if insert_pos == new_fruits.len() {
        new_fruits.push_back(fruit.as_str());
    }
    let fruits = new_fruits;

    println!("After inserting '{fruit}' at position {insert_pos}:");
    for (i, item) in fruits.iter().enumerate() {
        if i != fruits.len() - 1 {
            println!("{i} {item} ");
        } else {
            println!("{i} {item} ");
        }
    }
}
