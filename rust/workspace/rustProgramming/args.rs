fn sum(nums: &[i32]) -> i32 {
    let mut result = 0;
    for num in nums {
        result += num;
    }

    result
}

fn main() {
    //no variatic args in rust
    let nums = [1,2,3,4,5];
    let result =  sum(&nums);
    println!("the sum is {}",result);
}
