/*
    392. Is Subsequence
    -> LeetCode {easy}

    Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
    A subsequence of a string is a new string that is formed from the original string by deleting some 
    (can be none) of the characters without disturbing the relative positions of the remaining characters. 
    (i.e., "ace" is a subsequence of "abcde" while "aec" is not).

    https://leetcode.com/problems/is-subsequence/description/
*/

use std::io;

fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() { return true }

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut found = 0;
    let mut sub_len = s.len();
    for i in 0..t.len() {
        if t[i] == s[found] {
            found += 1;
        }
        if found >= sub_len {
            break
        }
    }

    found==sub_len
}

fn main() {
    let mut s: String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Unable to read input 's'!");
    
    let mut t: String = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Unable to read input ''!");

    println!("res : {}", is_subsequence(s, t));
}