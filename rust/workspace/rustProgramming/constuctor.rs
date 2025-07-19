struct Person {
    first_name: String,
    last_name:  String,
    email:      String,
    active:        bool
}

impl Person {
    fn new(first_name: String, last_name: String, email: String) -> Self {
        Self {
           first_name,
            last_name,
            email,
            active: true,
        }
    }
    fn deactivate(&mut self){
        self.active = false;
    }
}

fn introduce_person () {
    let person = Person {
        first_name: String::from("John"),
        last_name:  String::from("Doe"),
        email:      String::from("blah@test.com"),
        active: true,
    };
    println!("Hello, my name is {} {}.", person.first_name, person.last_name);
}

fn create_user() {
    let mut new_person = Person::new(
        String::from("Jane"),
        String::from("Doe"),
        String::from("email2@email.com"),
    );
    println!("New user created: {} {}", new_person.first_name, new_person.last_name);
    // Deactivate the user
    new_person.deactivate();
}

fn main() {
    introduce_person();
    create_user();
}
