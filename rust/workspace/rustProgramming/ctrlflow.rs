fn main() {
    let proceed = true;
    if proceed {
        println!("proceeding");
    } else {
        print!("halting");
    }

    let height = 190;
    let age = 25;
    if height > 189 {
        println!("{} is greater",height);
    } else if height == 150 {
        println!("{} is tiny", height);
    } else {
        println!("{} is smaller", height);
    }

    if age < 10 {
        println!("you are a {} year old baby",age);
    } else if age == 40 {
        println!("you are me");
    } else if age > 18 {
        println!("time to grow up your {} now",age);
    }
}
