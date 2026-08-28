// we want largest number in 2 diff lists - simple ex
fn largest_number_base() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");
}

// we want largest number in 2 diff lists - extract to function
// define reusable fn
fn largest_number_fn_def(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// we want largest number in 2 diff lists - extract to function
// call reusable fn (lets us keep our main below clean or filled with all the examples)
fn largest_number_fn_call() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_number_fn_def(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_number_fn_def(&number_list);
    println!("The largest number is {result}");
}

// in this ex we get the largest i32 in 1 fn and largest char in the other
// all the code is basically duplicated except for fn names and sig types
fn largest_i32_generic_fn_base(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char_generic_fn_base(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_generic_call() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32_generic_fn_base(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char_generic_fn_base(&char_list);
    println!("The largest char is {result}");
}

// here we have a fn with the generic type T
// idea being the function is a generic over some Type (T)
// takes one param list which is a slice of values of type T
// returns reference to that same type T
// fn largest_generic_type<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }
//
// fn largest_generic_type_call() {
//     let number_list = vec![34, 50, 25, 100, 65];
//
//     let result = largest_generic_type(&number_list);
//     println!("The largest number is {result}");
//
//     let char_list = vec!['y', 'm', 'a', 'q'];
//
//     let result = largest_generic_type(&char_list);
//     println!("The largest char is {result}");
// }

struct Point<T> {
    x: T,
    y: T,
}

struct PointdiffTypes<T, U> {
    x: T,
    y: U,
}

// this would of course go at top of file
// in this struct ex x and y have to both be same type
fn struct_generic_type() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("int point x: {}\n float point x: {}", integer.x, float.x);
}

// in this struct ex x and y from our struct can be diff types
fn struct_generic_diff_type() {
    let both_integer = PointdiffTypes { x: 5, y: 10 };
    let both_float = PointdiffTypes { x: 1.0, y: 4.0 };
    let integer_and_float = PointdiffTypes { x: 5, y: 4.0 };
    println!(
        "int point x: {}\nfloat point x: {}\nmixed point x: {}",
        both_integer.x, both_float.x, integer_and_float.x
    );
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// here enum can also have diff generic data types as with Result
fn enum_generic_diff_type() {
    let ok_result: Result<i32, &str> = Result::Ok(42);

    match ok_result {
        Result::Ok(value) => println!("Got value: {}", value),
        Result::Err(e) => println!("Got error: {}", e),
    }

    let err_result: Result<i32, &str> = Result::Err("something went wrong");

    match err_result {
        Result::Ok(value) => println!("Got value: {}", value),
        Result::Err(e) => println!("Got error: {}", e),
    }
}

pub fn largest_number() {
    largest_number_base();
    largest_number_fn_call();
    largest_generic_call();
    //largest_generic_type_call();
    struct_generic_type();
    struct_generic_diff_type();
    enum_generic_diff_type();
}
