// Lifetimes are Rust's way of making sure references don't outlive the data they point to
// They're like expiration dates on references

// This would be BAD (Rust prevents it):
// fn main() {
//     let reference;
//     {
//         let data = String::from("hello");
//         reference = &data;  // reference points to data
//     }  // data is GONE here!
//
//     // println!("{}", reference);  // Using reference now = disaster!
// }

// The 'a says: "the returned reference lives as long as the input reference"
fn first_word<'a>(s: &'a str) -> &'a str {
    match s.find(' ') {
        Some(i) => &s[0..i],
        None => s,
    }
}

// The `'a` (read as "lifetime a") tells Rust that the returned `&str` won't outlive the input `&str`. If `sentence` goes away, `word` must go away first.
pub fn lifetime_basic_ex() {
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("{}", word); // "hello"
}

// When Lifetimes are needed
// The function could return either `s1` or `s2`
// The `'a` tells Rust "the result lives only as long as BOTH inputs are valid."
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

pub fn lifetimes_longer_ex() {
    let a = String::from("short");
    let b = String::from("much longer");

    let result = longer(&a, &b);
    println!("{}", result);
}

// Two different lifetimes - we only care about one for the return
// announcement has lifetime 'a, data has 'b, since we return data, only 'b matters for the return value
fn announce_and_return<'a, 'b>(announcement: &'a str, data: &'b str) -> &'b str {
    println!("Announcement: {}", announcement);
    data // only data is returned, so only 'b matters for return
}

pub fn lifetimes_different_ex() {
    let my_data = String::from("important data");
    let result;

    {
        let announcement = String::from("Hello!");
        result = announce_and_return(&announcement, &my_data);
        // announcement can die here, we don't return it
    }

    println!("{}", result); // "important data" - still valid!
}
