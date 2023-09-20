/*
    26. Remove Duplicates from Sorted Array
    -> LeetCode {esay}

    Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that 
    each unique element appears only once. The relative order of the elements should be kept the same. 
    Then return the number of unique elements in nums.

    https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
*/

use std::io;

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    match nums.is_empty() {
        true => return 0,
        false => {
            
            let mut prev = 0;
            for idx in 1..nums.len() {
                if nums[prev] != nums[idx] {
                    prev += 1;
                    nums[prev] = nums[idx];
                }
            }
            (prev + 1) as i32
        }
    }
}


fn main() {
    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Unable to read input!");

    let mut nums: Vec<i32> = input_line.trim()
                                    .split_whitespace()
                                    .filter(|s| !s.is_empty())
                                    .map(|s| s.parse::<i32>().unwrap())
                                    .collect();
    
    // println!("nums : {:?}", nums);
    remove_duplicates(&mut nums);
}