/*
	217. Contains Duplicate
	-> LeetCode {easy}
	
	Given an integer array nums, return true if any value appears at least twice in the array, 
	and return false if every element is distinct.
*/

use std::io::{self, BufRead};
use std::collections::HashSet;

fn contains_duplicate_opt(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    for num in nums {
        if !set.insert(num) {
            return true;
        }
    }
    false
}

fn contains_duplicate_naive(nums: Vec<i32>) -> bool {
	let mut seen_numbers: Vec<i32> = Vec::<i32>::new();
	for i in 0..nums.len(){
		if seen_numbers.contains(&nums[i]) {
			return true;
		}
		else {
			seen_numbers.push(nums[i]);
		}
	}
	false
}

fn main () {
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();
	let input_line = match stdin_iterator.next() {
							Some(Ok(line)) => line,
							_ => {
								println!("No input provided!");
								return ;
							}
		
						};
	let nums: Vec<i32> = input_line.trim()
									.split(' ')
									.filter(|s| !s.is_empty())
									.map(|s| s.to_string().parse::<i32>().unwrap())
									.collect();
									
	let res: bool = contains_duplicate_opt(nums);
	println!("{}", res);
}