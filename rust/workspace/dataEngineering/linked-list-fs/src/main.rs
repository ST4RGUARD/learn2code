/*
Tasty fruit salad, user input for adding fruit at any position in the LinkedList after shuffling
Choose a random fruit in the salad
User name fruit to remove from any posiiton displaying name and list after removal
*/

use rand::seq::SliceRandom;
use rand::prelude::*;
use std::collections::LinkedList;
use std::io::{self, Write};

fn add_fruit(fruits: &mut LinkedList<String>) {
    print!("\nenter a fruit to add: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line!");
    
    let fruit = input.trim().to_string();
    if fruit.is_empty() {
        println!("no fruit entered, skipping!!");
        return;
    }

    print!("enter a fruit position: ");
    io::stdout().flush().unwrap();
    let mut position = String::new();
    io::stdin().read_line(&mut position).expect("failed to read line!");
    println!();
    let position: usize = match position.trim().parse() {
        Ok(num) if num <= fruits.len() => num,
        _ => {
            println!("invalid position, skipping!!");
            return;
        }
    };

    let mut new_fruits = LinkedList::new();
    
    for (i, item) in fruits.iter().enumerate() {
        if i == position {
            new_fruits.push_back(fruit.clone());
        }
        new_fruits.push_back(item.clone());
    }
    
    if position == fruits.len() {
        new_fruits.push_back(fruit);
    }
    
    *fruits = new_fruits;
}

fn add_more_fruits(fruits: &mut LinkedList<String>) {
    println!("adding more fruits...\n");
    fruits.push_front("Pomegranate".to_string());
    fruits.push_back("Fig".to_string());
    fruits.push_back("Cherry".to_string());
}

fn choose_fruit(fruits: &LinkedList<String>) {
    let mut rng = rand::rng();
    let fruit_vec: Vec<_> = fruits.iter().collect();
    if let Some(choice) = fruit_vec.choose(&mut rng) {
        println!("\nrandom fruit: {choice}");
    } else {
        println!("The list is empty");
    }
}

fn shuffle_fruits(fruits: &mut LinkedList<String>) {
    println!("\nshuffling fruits...\n");
    let mut rng = rand::rng();
    let mut fruit_vec: Vec<_> = fruits.iter().cloned().collect();
    fruit_vec.shuffle(&mut rng);
    *fruits = fruit_vec.into_iter().collect();
}

fn create_fruit_list() -> LinkedList<String> {
    let mut fruits = LinkedList::new();
    fruits.push_back("Arbutus".to_string());
    fruits.push_back("Loquat".to_string());
    fruits.push_back("Strawberry Tree Berry".to_string());
    fruits
}

fn print_fruits(fruits: &LinkedList<String>) {
    println!("-----------------F R U I T S -----------------");
    for (i, item) in fruits.iter().enumerate() {
        println!("{i} {item} ");
    }
}

fn main() {
    let mut fruits = create_fruit_list();
    print_fruits(&fruits);
    shuffle_fruits(&mut fruits);
    print_fruits(&fruits);
    choose_fruit(&fruits);
    add_more_fruits(&mut fruits);
    print_fruits(&fruits);
    add_fruit(&mut fruits);
    print_fruits(&fruits);
}

