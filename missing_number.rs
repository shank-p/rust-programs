/*
	268. Missing Number
	-> LeetCode {easy}
	
	Given an array nums containing n distinct numbers in the range [0, n], return the only 
	number in the range that is missing from the array.
*/

use std::io::{self, BufRead};
use std::convert::TryInto;

fn missing_number(nums: Vec<u32>) -> u32 {
	let nums_length: u32 = nums.len().try_into().unwrap();
	let expected_sum: u32 = (nums_length * (nums_length+1))/2;
	let nums_sum: u32 = nums.iter().sum();
	
	let missing_num: u32 = expected_sum - nums_sum;
	missing_num
}

fn main() {
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();
	
	let input_line: String = match stdin_iterator.next() {
						Some(Ok(line)) => line,
						_ => return
					};
	let nums: Vec<u32> = input_line.trim_end()
									.split(' ')
									.filter(|s| !s.is_empty())
									.map(|s| s.to_string().parse::<u32>().unwrap())
									.collect();

	let missing_num = missing_number(nums);
	println!("{}", missing_num);
	
}