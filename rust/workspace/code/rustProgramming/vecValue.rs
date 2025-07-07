fn get_item() {
    let index = 3;
    let vec = vec![1,2,3,4,5];

    let value = vec.get(index).unwrap();

    println!("value at index {} is {:?}", index, value);
}

fn main() {
    let vec = vec![1,2,3,4,5];
    get_item();

    let third = vec[2];
    let last = vec.last().unwrap();
    println!("3rd value: {} last value: {}", third, last);
}
