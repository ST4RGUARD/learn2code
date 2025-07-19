fn process_numbers(numbers: &[i32]) {
    let mut sum = 0;
    for number in numbers {
        sum += number;
    }

    println!("the sum of the numbers is {}",sum);

    if sum % 2 == 0 {
        println!("sum is even");
    } else {
        println!("sum is odd");
    }
}

fn main() {
   process_numbers(&[1,3,5]); 
}
