/*
    1758. Minimum Changes To Make Alternating Binary String
    -> LeetCode

    You are given a string s consisting only of the characters '0' and '1'. In one operation, you can change any 
    '0' to '1' or vice versa.
    The string is called alternating if no two adjacent characters are equal. For example, the string "010" is 
    alternating, while the string "0100" is not.
    Return the minimum number of operations needed to make s alternating.

    https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/description/?envType=daily-question&envId=2023-12-24
*/

use std::io;
use std::cmp;

pub fn min_operations(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let (mut even_ones, mut even_zeroes, mut odd_ones, mut odd_zeroes): (i32, i32, i32, i32) = (0, 0, 0, 0);
    for (idx, c) in s.chars().enumerate() {
        if idx%2 == 0 && c == '0' {
            even_zeroes += 1;
        } else if idx%2 == 0 && c == '1' {
            even_ones += 1;
        } else if c == '0' {
            odd_zeroes += 1;
        } else {
            odd_ones += 1;
        }
    }
    let pattern_1: i32 = even_zeroes + odd_ones;  //0101
    let pattern_2: i32 = even_ones + odd_zeroes;  //1010
    cmp::min(s.len() as i32 - pattern_1, s.len() as i32 - pattern_2)
}

fn main() {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Unabel ro read input `s`");
    let s = s.trim().to_string();
    let res = min_operations(s);
    println!("res : {res}");
}