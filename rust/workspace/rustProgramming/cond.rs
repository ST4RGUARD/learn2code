fn main() {
    let may_num: Option<Option<()>> = Some(None);
    //let may_num = Some(42);

    if let Some(num) = may_num {
        println!("the number is: {:?}",num);
    } else {
        println!("there is no number");
    }
}
