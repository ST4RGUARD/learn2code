use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let count: usize = lines
        .next()
        .expect("Missing first line")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Not a number");

    let mut turn_cnt = Vec::new();

    for _ in 0..count {
        let line = lines
            .next()
            .expect("Missing line")
            .expect("Failed to read line");

        let mut nums: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().expect("Not a number"))
            .collect();

        let mut turns = 0;
        'counting_turns: loop {
            if nums[0] == nums[1] || nums[0] == nums[2] || nums[1] == nums[2] {
                turn_cnt.push(turns);
                break 'counting_turns;
            } else if nums[0] > nums[1] && nums[0] > nums[2] {
                if nums[1].max(nums[2]) == nums[1] {
                    nums[2] += 1;
                } else if nums[1].max(nums[2]) == nums[2] {
                    nums[1] += 1;
                }
                nums[0] -= 1;
                turns += 1;
            } else if nums[1] > nums[0] && nums[1] > nums[2] {
                if nums[0].max(nums[2]) == nums[0] {
                    nums[2] += 1;
                } else if nums[0].max(nums[2]) == nums[2] {
                    nums[0] += 1;
                }
                nums[1] -= 1;
                turns += 1;
            } else if nums[2] > nums[0] && nums[2] > nums[1] {
                if nums[0].max(nums[1]) == nums[0] {
                    nums[1] += 1;
                } else if nums[0].max(nums[1]) == nums[1] {
                    nums[0] += 1;
                }
                nums[2] -= 1;
                turns += 1;
            } else {
                println!("shouldnt hit this");
            }
        }
    }
    for cnt in &turn_cnt {
        println!("{}", cnt);
    }
}
