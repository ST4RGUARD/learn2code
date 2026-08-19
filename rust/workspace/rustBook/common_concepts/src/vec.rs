// store values of same type
pub fn vects() {
    // unknown contents or size
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("vec1: {:?}", v);

    // known contents
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // print out immutable elements of vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // update mutable elements of vector
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
