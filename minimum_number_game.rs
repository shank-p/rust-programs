/*
    2974. Minimum Number Game
    -> Leetcode

    You are given a 0-indexed integer array nums of even length and there is also an empty array arr. 
    Alice and Bob decided to play a game where in every round Alice and Bob will do one
    move. The rules of the game are as follows:
    - Every round, first Alice will remove the minimum element from nums, and then Bob
    does the same.
    - Now, first Bob will append the removed element in the array arr, and then Alice 
    does the same.
    - The game continues until nums becomes empty.
    Return the resulting array arr.    

    https://leetcode.com/contest/weekly-contest-377/problems/minimum-number-game/
*/

use std::io;

pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    for i in (0..sorted_nums.len()).step_by(2) {
        sorted_nums[i]   = sorted_nums[i+1] + sorted_nums[i];
        sorted_nums[i+1] = sorted_nums[i] - sorted_nums[i+1];
        sorted_nums[i]   = sorted_nums[i] - sorted_nums[i+1]; 
    }
    sorted_nums
}

fn main() {
    let mut nums = String::new();
    io::stdin()
        .read_line(&mut nums)
        .expect("Unable to read input `nums`");
    let nums: Vec<i32> = nums.split_whitespace()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.parse().unwrap())
                            .collect();
    
    let res: Vec<i32> = number_game(nums);
    println!("res : {:?}", res);
}