mod arrays;
mod conditional;
mod constants;
mod expression;
mod functions;
mod loops;
mod shadowing;
mod slice;
mod structs;
mod tuples;
mod vars;

fn main() {
    vars::mutability();
    constants::constants();
    shadowing::shadowing();
    tuples::tuples();
    tuples::tup_direct_access();
    arrays::arrays();
    arrays::array_element_access();
    functions::another_function(5);
    functions::plus_one(3);
    expression::expression();
    conditional::conditional();
    loops::loop_label();

    let mut s = String::from("hello world");

    let word = slice::first_word(&s);
    println!("The first word len is {}", word);
    s.clear();

    let mut user1 = structs::User::new(
        "someusername".to_string(),
        "someone@example.com".to_string(),
    );

    user1.email = "changed@example.com".to_string();
}
