fn strings() {
    let s = String::new();

    println!("STRING: {}", s);
    let data = "initial contents";

    let s = data.to_string();
    println!("STRING: {}", s);

    let s = String::from("initial contents");
    println!("STRING: {}", s);

    let s = "initial contents".to_string();
    println!("STRING: {}", s);

    // push_str doesnt take ownership if we want to reuse s
    let mut s = String::from("foo");
    let s2 = "test";
    s.push_str(s2);
    println!("STRING: {}", s);
    println!("STRINGS2: {}", s2);

    // push a single char
    let mut s = String::from("lo");
    s.push('l');
    println!("STRING: {}", s);

    // + operator to add strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("STRING: {}", s3);

    // format to concat
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("STRING: {}", s);

    // iterate strings
    for c in "Зд".chars() {
        println!("{c}");
    }

    // show bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

pub fn string_ex() {
    strings();
}
