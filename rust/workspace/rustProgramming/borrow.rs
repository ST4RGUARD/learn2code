fn own_str(s: &String) {
    println!("{}",s);
}

fn own_int(x: i32) -> i32{
    x + 1
}

fn own_vec(mut vector: Vec<i32>) {
    vector.push(10);
    println!("{:?}",vector);
}

fn main() {
    //copy : i32 bool
    let x = 5;  // x owns 5
    let y = x;  // x is copied to y
    let my_vec = vec![1,2,3,4,5];
    let my_int = 10;
    let my_str = String::from("Hello World");
    let new_str = my_str;  // my_str is moved 
    println!("x: {} y: {}",x,y);
    //this compiles
    own_int(my_int);
    println!("my int {}",my_int);

    own_str(&my_str);  // borrow my_str immutability doesn't own it my_str can still be used
    // own_str(&mut my_str) and own_str(s: &mut String) for a mutable borrow, only one mut borrow
    // at a time
    println!("my str {}",new_str); // new_str owns my_str
    //println!("{:?}",my_str);
    //this will not compile
    //don't know size of strings unlike int

    own_vec(my_vec);
    //also borrowed/moved will not compile
    //println!("{:?}",my_vec);

}
