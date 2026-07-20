// want to ensure that you always have a fixed number of elements. An array isn’t as flexible as the vector type, though.

pub fn arrays() {
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("a: {:?}", a);
    println!("a: {:?}", months);
}

pub fn array_element_access() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("first: {first}, second: {second}");
}
