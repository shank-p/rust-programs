/*
    1. Two Sum

    Given an array of integers nums and an integer target, return indices of the two numbers such that 
    they add up to target.

    You may assume that each input would have exactly one solution, and you may not use the same element
    twice.
    You can return the answer in any order.
*/

use std::io::{self, BufRead};
use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hm = HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        match hm.get(&num) {
            Some(&j) => return vec![i as i32, j as i32],
            None => {
                hm.insert(target - num, i);
            },
        }
    }
    unreachable!();
}

// fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut differences: HashMap<i32, i32> = HashMap::new();
//     let start_idx;
//     for (idx, value) in nums.iter().enumerate() {
//         match differences.contains_key(&value) {
//             true => {
//                 start_idx = differences.get(&value).unwrap();
//                 return vec![*start_idx as i32, idx as i32];
//             },
//             false => {
//                 differences.insert(target-value, idx as i32);
//             },
//         };
//     }
//     vec![-1, -1]
// }

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let input_line: String = match stdin_iterator.next() {
                                Some(Ok(line)) => line,
                                _ => {
                                        println!("Error reading vector input!");
                                        return ;
                                },
                            };
    let nums: Vec<i32> = input_line.trim()
                                    .split(' ')
                                    .filter(|s| !s.is_empty())
                                    .map(|s| s.to_string().parse::<i32>().unwrap())
                                    .collect();
    
    let target: i32 = match stdin_iterator.next() {
                        Some(Ok(val)) => val.trim().parse::<i32>().unwrap(),
                        _ => {
                                println!("Error reading target input!");
                                return ;
                        }
                    };
    
    let result = two_sum(nums, target);
    println!("result : {:?}", result);
}