use std::collections::HashMap;

fn main() {
    let mut items: HashMap<String, String> = HashMap::new();
    items.insert(String::from("One"), String::from("Book"));
    items.insert(String::from("Two"), String::from("Keyboard"));
    items.insert(String::from("Three"), String::from("Sunnys"));

    let keyboard = items.get("Two").unwrap();
    println!("{:?}",keyboard);

    items.remove("Three");
    println!("{:?}",items.get("Three"));

    for (k, v) in &items {
        println!("key: {} value: {}", k, v);
    }
}
