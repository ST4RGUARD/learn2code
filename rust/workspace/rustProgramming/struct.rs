struct Person {
    first_name: String,
    last_name:  String,
    email:      String,
    age:        u8,
}

fn introduce_person () {
    let person = Person {
        first_name: String::from("John"),
        last_name:  String::from("Doe"),
        email:      String::from("blah@test.com"),
        age:        30,
    };
    println!("Hello, my name is {} {}.", person.first_name, person.last_name);
    println!("My email is {} and my age is {}.", person.email, person.age);
}

fn main() {
    introduce_person();
}
