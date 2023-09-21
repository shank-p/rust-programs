/*
    704. Binary Search
    -> LeetCode {easy}

    Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. 
    If target exists, then return its index. Otherwise, return -1.
    You must write an algorithm with O(log n) runtime complexity.

    https://leetcode.com/problems/binary-search/description/
*/

use std::io;
use std::cmp::Ordering;

fn search(nums: Vec<i32>, target: i32) -> i32 {
    
    let (mut left, mut right) = (0, nums.len());
    let mut mid;
    while left < right {
        mid = (left + right)/2 as usize;
        match nums[mid].cmp(&target) {
            Ordering::Equal => return mid as i32,
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }
    return -1;
}

fn main() {
    let mut target: String = String::new();
    io::stdin()
        .read_line(&mut target)
        .expect("Unable to read input 'target' !");
    let target: i32 = target.trim().parse().unwrap();

    let mut input_line: String = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Unable to read input 'nums' !");
    let nums: Vec<i32> = input_line.trim()
                        .split_whitespace()
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();

    println!("res : {}", search(nums, target));
}