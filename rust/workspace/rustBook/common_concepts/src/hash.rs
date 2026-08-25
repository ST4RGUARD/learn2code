// hashmap keys have same type, values have same type
//

use std::collections::HashMap;

fn create_hashmap() -> HashMap<String, i32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("hash {:?}", scores);
    println!("team_name {}", score);

    scores
}

fn print_kv(scores: &HashMap<String, i32>) {
    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

fn overwrite_value(scores: &mut HashMap<String, i32>) {
    // let mut hashmap = scores;
    // above only needed iw we take ownership and work with a copy
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("scores: {:?}", scores);
}

pub fn get_hashmaps() {
    // if we want to modify the original hashmap - mut
    let mut hashmap = create_hashmap();
    print_kv(&hashmap);
    overwrite_value(&mut hashmap);
}
