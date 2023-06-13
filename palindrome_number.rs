/*
    9. Palindrome Number
    -> LeetCode {easy}

    Given an integer x, return true if x is a palindrome, and false otherwise.
 */

use std::io;

#[allow(unused_variables)]
fn is_palindrome(x: i32, length: usize) -> bool {
    for i in 0..length/2{
        println!("i {} len {}", i, length);
    }
    return true
}

fn main () {
    let mut input_num = String::new();
    io::stdin().read_line(&mut input_num).expect("Error reading input!");

    let num_length = input_num.len();
    let input_num: i32 = match input_num.trim().parse() {
                            Ok(num) => num,
                            Err(_) => 1,
                        };
    is_palindrome(input_num, num_length);
}   