/*
    343. Integer Break
    -> LeetCode {medium}

    Given an integer n, break it into the sum of k positive integers, where k >= 2, and maximize the product of 
    those integers. Return the maximum product you can get.

    https://leetcode.com/problems/integer-break/description/
*/

use std::io;

fn integer_break(n: i32) -> i32 {
    let q = n/3;
    match q {
        0 => return 1,  // either n==1 or n==2.
        1 => {
            match n % 3 {
                0 => return 2, // n==3
                1 => return 4, // n==4
                _ => return 6, // n==5
            }
        },
        _ => {
            match n % 3 {
                0 => return i32::pow(3, q as u32), 
                1 => return i32::pow(3, q as u32 -1)*4, 
                _ => return i32::pow(3, q as u32)*2, 
            }
        },
    }
}

fn main() {
    let mut n: String= String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Unable to read input 'n' !");
    let n: i32 = n.trim().parse::<i32>().unwrap();

    let res = integer_break(n);
    println!("result : {res}");
}