/*
|   Pattern       |       Use Case                             |
|-----------------|--------------------------------------------|
| Basic trait     | Define shared behavior for different types |
| Default impl    | Provide optional default behavior          |
| `&impl Trait`   | Accept any type with that trait            |
| `-> impl Trait` | Return any type with that trait            |
| `<T: Trait>`    | Generic with constraints                   |
| `T: A + B`      | Require multiple                           |
*/

// Define a trait - shared behavior
// Dog and Cat implement the same trait, but each does it their own way.

trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// Dog agrees to the Speak contract
impl Speak for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

// Cat agrees to the Speak contract
impl Speak for Cat {
    fn speak(&self) {
        println!("{} says: Meow!", self.name);
    }
}

pub fn talking_animals_trait_ex() {
    let dog = Dog {
        name: String::from("Rex"),
    };
    let cat = Cat {
        name: String::from("Whiskers"),
    };

    dog.speak();
    cat.speak();
}

// Default or override

trait Greet {
    fn greet(&self) {
        // Default behavior - types can use this or override it
        println!("Hello there!");
    }
}

struct Friendly;
struct Formal;

// Uses the default
impl Greet for Friendly {}

// Overrides the default
impl Greet for Formal {
    fn greet(&self) {
        println!("Good day, sir.");
    }
}

pub fn greeting_default_or_override_trait_ex() {
    let f = Friendly;
    let r = Formal;

    f.greet();
    r.greet();
}

// Trait as function parameter
// The function doesn't care about the exact type—only that it can `Describe` itself.

trait Describe {
    fn describe(&self) -> String;
}

struct Car {
    brand: String,
}

struct Bike {
    color: String,
}

impl Describe for Car {
    fn describe(&self) -> String {
        format!("A {} car", self.brand)
    }
}

impl Describe for Bike {
    fn describe(&self) -> String {
        format!("A {} bike", self.color)
    }
}

// This function accepts ANYTHING that implements Describe
fn print_description(item: &impl Describe) {
    println!("{}", item.describe());
}

pub fn function_parameter_trait_ex() {
    let car = Car {
        brand: String::from("Toyota"),
    };
    let bike = Bike {
        color: String::from("red"),
    };

    print_description(&car);
    print_description(&bike);
}

// Returning traits
// The function promises to return *something* that can summarize itself, without specifying the exact type.

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Article: {}", self.title)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// Returns "something that implements Summary"
fn create_article() -> impl Summary {
    Article {
        title: String::from("Rust is great!"),
    }
}

pub fn function_return_trait_ex() {
    let item = create_article();
    println!("{}", item.summarize());
}

// Generics with traits - trait bounds
// The `<T: Printable>` says "T can be any type, but it MUST implement Printable."

trait Printable {
    fn format(&self) -> String;
}

struct Page {
    content: String,
}

impl Printable for Page {
    fn format(&self) -> String {
        format!("=== PAGE ===\n{}", self.content)
    }
}

// T must implement Printable - this is a "trait bound"
fn print_twice<T: Printable>(item: T) {
    println!("{}", item.format());
    println!("{}", item.format());
}

pub fn trait_bounds_ex() {
    let page = Page {
        content: String::from("Hello world"),
    };
    print_twice(page);
}

// Multiple traits
// The `+` lets you require multiple traits. The type must implement ALL of them.

trait Run {
    fn run(&self);
}

trait Jump {
    fn jump(&self);
}

struct Athlete {
    name: String,
}

impl Run for Athlete {
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}

impl Jump for Athlete {
    fn jump(&self) {
        println!("{} is jumping!", self.name);
    }
}

// Requires BOTH traits
fn exercise<T: Run + Jump>(person: &T) {
    person.run();
    person.jump();
}

pub fn multiple_traits_ex() {
    let athlete = Athlete {
        name: String::from("Bob"),
    };
    exercise(&athlete);
}
