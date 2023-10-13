/*
    66. Plus One
    -> LeetCode {easy}

    You are given a large integer represented as an integer array digits, where each digits[i] is the 
    ith digit of the integer. The digits are ordered from most significant to least significant in 
    left-to-right order. The large integer does not contain any leading 0's.
    Increment the large integer by one and return the resulting array of digits.

    https://leetcode.com/problems/plus-one/description/
*/

use std::io;

fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut icremented_digits = digits;
    for i in (0..icremented_digits.len()).rev() {
        match icremented_digits[i] {
            9 => {
                icremented_digits[i] = 0;
            },
            _ => {
                icremented_digits[i] += 1;
                return icremented_digits
            }
        };
    }
    icremented_digits.insert(0, 1);
    return icremented_digits
}

fn main() {
    let mut digits: String = String::new();
    io::stdin()
        .read_line(&mut digits)
        .expect("Unable to read `digits` !");

    let digits: Vec<i32> = digits.trim()
                                .split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect();
    println!("result : {:?}", plus_one(digits));
}