/*
    1913. Maximum Product Difference Between Two Pairs
    -> LeetCode {easy}

    The product difference between two pairs (a, b) and (c, d) is defined as (a * b) - (c * d).
    Given an integer array nums, choose four distinct indices w, x, y, and z such that the 
    product difference between pairs (nums[w], nums[x]) and (nums[y], nums[z]) is maximized.
    Return the maximum such product difference.

    https://leetcode.com/problems/maximum-product-difference-between-two-pairs/description/?envType=daily-question&envId=2023-12-18
*/

use std::io;

pub fn max_product_difference(nums: Vec<i32>) -> i32 {
    let (mut w, mut x) = (std::i32::MIN, std::i32::MIN);
    let (mut y, mut z) = (std::i32::MAX, std::i32::MAX);
    for num in nums {
        if num > w {
            x = w;
            w = num;
        } else if num > x {
            x = num;
        }
        
        if num < y {
            z = y;
            y = num;
        } else if num < z {
            z = num;
        }
    }

    (w*x) - (y*z)
}

fn main() {
    let mut nums: String = String::new();
    io::stdin()
        .read_line(&mut nums)
        .expect("Unable to read input `nums`");
    let nums: Vec<i32> = nums.split_whitespace()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect();
    
    let res =  max_product_difference(nums);
    println!("res : {res}");
}
