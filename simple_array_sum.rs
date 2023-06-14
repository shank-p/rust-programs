/*
	Simple Array Sum
	-> HackerRank {easy}
	
	Print the sum of the entered Array.
*/

use std::io::{self, BufRead};

fn main() {
	let stdin = io::stdin();
	let mut stdin_iterator = stdin.lock().lines();
	
	let nums_length = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
	if nums_length != -1 {
		let nums: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
								.trim_end()
								.split(' ')
								.map(|s| s.to_string().parse::<i32>().unwrap())
								.collect();
		
		let sum: i32 = nums.iter().sum();
		println!("{}", sum);
	}
	
	
	
}