/*
    75. Sort Colors
    -> leetcode (medium)
    https://leetcode.com/problems/sort-colors/description/?envType=daily-question&envId=2024-06-12
*/

use std::io;

pub fn sort_colors(nums: &mut Vec<i32>) {
    let (mut red_end, mut blue_start): (usize, usize) = (0, nums.len()-1);
    let mut idx: usize = 0;
    while idx<=blue_start && idx<nums.len(){
        if nums[idx] == 1 {
            idx += 1;
            continue;
        } else if nums[idx] == 0 {
            nums.swap(idx, red_end);
            red_end += 1;
            idx += 1;
        } else if nums[idx] == 2 {
            nums.swap(idx, blue_start);
            if blue_start>0 {
                blue_start -= 1;
            } else { break }
        }
        println!("-->> res : {nums:?}");
    }
}

fn main() {
    let mut nums = String::new();
    io::stdin()
        .read_line(&mut nums)
        .expect("Unable to read input `nums`!");
    let mut nums = nums.split_whitespace()
                    .map(|s| s.to_string().parse::<i32>().unwrap())
                    .collect();
    
    sort_colors(&mut nums);
}