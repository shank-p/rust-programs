/*
    231. Power of Two
    -> LeetCode {easy}

    Given an integer n, return true if it is a power of two. Otherwise, return false.
    An integer n is a power of two, if there exists an integer x such that n == 2x.
    
    https://leetcode.com/problems/power-of-two/description/
*/

use std::io;
use std::cmp::Ordering;

pub fn is_power_of_two(n: i32) -> bool {
    
    // Based on the instace that a number which is a power of 2, and its previuos number 
    // have no bit in common. Hence 'AND'ing both will result in all 0.
    // ex. 7 => 0 1 1 1
    //     8 => 1 0 0 0
    //    res : 0 0 0 0 

    n > 0 && (n & (n - 1)) == 0
}

fn new(n: i32) -> bool {
    let mut rem;
    loop {
        rem = n/2;
        match rem%2 {
            0 => return 
        };
    }
}

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Unable to read input `n` !");
    let n: i32 = n.trim().parse().unwrap();

    println!("result : {}", is_power_of_two(n));
}