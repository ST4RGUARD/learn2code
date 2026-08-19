// if you know the types
fn validate_cell_type() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // print values
    for i in &row {
        println!("{:?}", i);
    }
    // retrieve values
    // match
    //
    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => println!("Integer: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }
    // if let
    //
    for i in &row {
        if let SpreadsheetCell::Int(v) = i {
            println!("Int: {}", v);
        } else if let SpreadsheetCell::Float(v) = i {
            println!("Float: {}", v);
        } else if let SpreadsheetCell::Text(v) = i {
            println!("Text: {}", v);
        }
    }
    // as methods more idiomatic
    //
    impl SpreadsheetCell {
        // Returns Some(value) if Int, None otherwise
        fn as_int(&self) -> Option<i32> {
            match self {
                SpreadsheetCell::Int(v) => Some(*v),
                _ => None,
            }
        }

        fn as_float(&self) -> Option<f64> {
            match self {
                SpreadsheetCell::Float(v) => Some(*v),
                _ => None,
            }
        }

        fn as_text(&self) -> Option<&str> {
            match self {
                SpreadsheetCell::Text(v) => Some(v),
                _ => None,
            }
        }
    }

    // Usage:
    for i in &row {
        if let Some(num) = i.as_int() {
            println!("Got integer: {}", num);
        }
    }
}

pub fn stubby() {
    validate_cell_type();
}
