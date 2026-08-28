// panic intent is unrucoverable error
// unwinds and cleans up the stack by default
// abort* - stop execution immediately
// *when program size matters
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn panic_ex() {
    panic!("crash and burn");
}

// if we want to handle an error
// i.e error doesnt warrant program termination
// we use result
fn result_ex() {
    // attempt to open file
    let greeting_file_result = File::open("hello.txt");

    // match on success / ok file, error - err
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}

fn error_kind_ex() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // if io::error method kind is of type notfound then we try to File::create
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}

fn unwrap_ex() {
    // if Result value is OK unwrap will return the value inside ok
    // if it is err it will unwrap the panic macro
    let greeting_file = File::open("hello.txt").unwrap();
}

fn expect_ex() {
    // expect lets us choose the err msg
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");
}

// progogating errors

// the ? at the end of the File::open returns the value inside Ok to username_file
// if error occurs, the ? operator will return early out of the whole function and give any Err value to the calling code
//jsame thing applies to the ? at the end of the read_to_string call.
fn operator_question_mark_ex() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn shorter_operator_question_mark_ex() -> Result<String, io::Error> {
    let mut username = String::new();

    // have ? operator on open and read_to_string
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn fs_read_to_string() -> Result<String, io::Error> {
    // std lib gives us fs read_to_string
    // opens file, reads contents, puts contents in string returns it
    fs::read_to_string("hello.txt")
}

pub fn errors() {
    //panic_ex();
    //result_ex();
    //error_kind_ex();
    //unwrap_ex();
    //expect_ex();
    //operator_question_mark_ex();
    //shorter_operator_question_mark_ex();
    //fs_read_to_string();
}
