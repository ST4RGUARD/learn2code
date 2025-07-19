fn main() {
    let _v: () = ();
    let v = (2, 3);

    assert_eq!(v, implicity_ret_unit());

    println!("Success");
}

fn implicity_ret_unit() {
    println!("I will return a ()");
}

fn test(){
    print('this')
        go

}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}
