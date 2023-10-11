/*
    34. Find First and Last Position of Element in Sorted Array
    -> LeetCode {medium}

    Given an array of integers nums sorted in non-decreasing order, find the starting and ending 
    position of a given target value. If target is not found in the array, return [-1, -1].
    You must write an algorithm with O(log n) runtime complexity.

    https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/
*/

use std::io;
use std::cmp::Ordering;

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    
    if nums.is_empty() {
        return vec![-1, -1]
    }
    let mut low = 0;
    let mut high = nums.len() -1;
    let mut mid;
    while low <= high {
        mid = (low+high)/2;
        match nums[mid].cmp(&target) {
            Ordering::Less => {
                low = mid+1;
            },
            Ordering::Greater => {
                if mid == 0 { break; }
                high = mid-1;
            },
            Ordering::Equal => {
                let mut left = -1;
                for i in 0..=mid {
                    if nums[i] == target {
                        left = i as i32;
                        break;
                    }
                } 
                let mut right = -1;
                for i in (mid..nums.len()).rev() {
                    if nums[i] == target {
                        right = i as i32;
                        break;
                    }
                }
                return vec![left, right];
            }
        }
    }
    vec![-1, -1]
}

fn main() {
    let mut nums = String::new();
    io::stdin()
        .read_line(&mut nums)
        .expect("Unable to read input for 'nums'!");
    let nums: Vec<i32> = nums.trim()
                            .split_whitespace()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect();

    let mut target = String::new();
    io::stdin()
        .read_line(&mut target)
        .expect("Unable to read input for 'target'!");
    let target = target.trim().parse::<i32>().unwrap();

    let res: Vec<i32> = search_range(nums, target);
    println!("result : {:?}", res);
}