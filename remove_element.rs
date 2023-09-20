/*
    27. Remove Element
    -> LeetCode {easy}

    Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. 
    The order of the elements may be changed. Then return the number of elements in nums which are not equal 
    to val.

    https://leetcode.com/problems/remove-element/description/
*/

use std::io;

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    match nums.is_empty() {
        true => return 0,
        false => {

            let mut prev = 0;
            for i in 0..nums.len() {
                if nums[i] != val {
                    if prev != i {
                        nums[prev] = nums[i];
                    } 
                    prev += 1;
                }
            }

            println!("nums: {:?}", nums);
            println!("stop : {}", prev as i32);
            prev as i32
        }
    }       
}

fn main() {

    let mut input_number: String = String::new();
    io::stdin()
        .read_line(&mut input_number)
        .expect("Unable to read input!");
    let val: i32 = input_number.trim().parse().unwrap();

    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Unable to read input!");
    let mut nums = input_line.trim()
                            .split_whitespace()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect();
    
    remove_element(&mut nums, val);
}