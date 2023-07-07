/*
    53. Maximum Subarray
    -> LeetCode {medium}

    Given an integer array nums, find the subarray with the largest sum, and return its sum.
*/

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let input_line: String = match stdin_iterator.next() {
                                Some(Ok(line)) => line, 
                                _ => return,
                            }; 

    let nums: Vec<i32> = input_line.trim_end()
                                    .split(' ')
                                    .filter(|s| !s.is_empty())
                                    .map(|s| s.to_string().parse::<i32>().unwrap())
                                    .collect();

    println!("nums: {:?}", nums);
    
    let mut cur_max: i32 = 0;
    let mut max: i32 = 0;

    for index in 0..nums.len(){
        cur_max = cur_max + nums[index];
        if cur_max > max {
            max = cur_max;
        }
        if cur_max < 0{
            cur_max = 0;
        }
    }

    println!("max: {}", max);
}
